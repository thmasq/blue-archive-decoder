extern crate alloc;

mod blue_archive_generated;
mod db_migrator;

use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;
use std::panic;
use wasm_bindgen::prelude::*;

// Internal struct to hold processed table data in memory
struct TableData {
    columns: Vec<String>,
    rows: Vec<Vec<Value>>,
}

#[wasm_bindgen]
pub struct Inspector {
    // In-memory storage of all processed tables (Name -> Data)
    tables: HashMap<String, TableData>,

    // View state for the currently selected table
    current_table_name: String,
    filtered_indices: Vec<usize>,

    last_query: String,
}

#[wasm_bindgen]
impl Inspector {
    #[wasm_bindgen(constructor)]
    pub fn new(file_data: Vec<u8>) -> Result<Inspector, String> {
        panic::set_hook(Box::new(|info| {
            web_sys::console::error_1(&format!("Rust Panic: {}", info).into());
        }));

        web_sys::console::log_1(&"Initializing DB...".into());

        // 1. Open the database temporarily
        let cursor = Cursor::new(file_data);
        let mut db = Database::new(cursor).map_err(|e| format!("Failed to open DB: {}", e))?;

        let mut tables = HashMap::new();

        // 2. Load and Process Virtual Tables (Flatbuffers)
        // We get the registry of loaders and execute them all immediately.
        let mut loaders = HashMap::new();
        db_migrator::register_loaders(&mut loaders);

        web_sys::console::log_1(&format!("Registered {} loaders", loaders.len()).into());

        for (name, loader) in loaders {
            web_sys::console::log_1(&format!("Loading Virtual Table: {}", name).into());

            match loader(&mut db) {
                Ok((cols, rows)) => {
                    tables.insert(
                        name,
                        TableData {
                            columns: cols,
                            rows,
                        },
                    );
                }
                Err(e) => {
                    web_sys::console::warn_1(&format!("Failed to load {}: {}", name, e).into());
                }
            }
        }

        // 3. Load Physical SQL Tables
        // Iterate over all actual tables in the DB.
        let sql_table_names = db
            .tables()
            .map_err(|e| format!("Failed to list tables: {}", e))?;

        for name in sql_table_names {
            // Avoid overwriting a Virtual table if one exists with the same name
            // (Unlikely given Schema naming conventions, but good safety).
            if !tables.contains_key(&name) {
                web_sys::console::log_1(&format!("Loading Physical Table: {}", name).into());

                let columns = db
                    .get_table_columns(&name)
                    .map_err(|e| format!("Failed to get columns for {}: {}", name, e))?;

                // Fetch All Rows
                let query = SelectQuery::new(&name);
                let raw_rows = db
                    .execute_query(&query)
                    .map_err(|e| format!("Failed to query data for {}: {}", name, e))?;

                // Convert to Vec<Vec<Value>>
                let rows: Vec<Vec<Value>> = raw_rows
                    .into_iter()
                    .map(|row| row.values().into_iter().cloned().collect())
                    .collect();

                tables.insert(name, TableData { columns, rows });
            }
        }

        web_sys::console::log_1(&"DB Init Complete.".into());

        Ok(Inspector {
            tables,
            current_table_name: String::new(),
            filtered_indices: Vec::new(),
            last_query: String::new(),
        })
    }

    /// Returns a list of all available processed tables
    pub fn get_tables(&self) -> Result<Box<[JsValue]>, String> {
        let mut names: Vec<String> = self.tables.keys().cloned().collect();
        names.sort();

        let js_array: Vec<JsValue> = names.into_iter().map(JsValue::from).collect();
        Ok(js_array.into_boxed_slice())
    }

    /// Selects a table for viewing.
    /// Since data is already in memory, this is now just a lookup.
    pub fn load_table(&mut self, table_name: &str) -> Result<usize, String> {
        if !self.tables.contains_key(table_name) {
            return Err(format!("Table not found: {}", table_name));
        }

        self.current_table_name = table_name.to_string();

        // Reset view to show all rows
        let row_count = self.tables.get(table_name).unwrap().rows.len();
        self.filtered_indices = (0..row_count).collect();

        self.last_query.clear();

        Ok(row_count)
    }

    /// String search filter with Iterative Filtering
    pub fn apply_filter(&mut self, query: &str) -> usize {
        let table_data = match self.tables.get(&self.current_table_name) {
            Some(t) => t,
            None => return 0,
        };

        let q_lower = query.to_lowercase();

        if query.is_empty() {
            // Reset to full set if we aren't already there
            if self.filtered_indices.len() != table_data.rows.len() {
                self.filtered_indices = (0..table_data.rows.len()).collect();
            }
            self.last_query.clear();
        } else {
            // Check optimization: Is the new query a refinement of the old one?
            // If yes, we can scan ONLY the currently filtered indices.
            let is_refinement =
                !self.last_query.is_empty() && q_lower.starts_with(&self.last_query);

            if is_refinement {
                // O(Filtered_Rows * Cols) - Iterative
                self.filtered_indices.retain(|&index| {
                    let row = &table_data.rows[index];
                    Inspector::row_matches(row, &q_lower)
                });
            } else {
                // O(Total_Rows * Cols) - Full Scan
                // Occurs on first search, backspace, or paste
                self.filtered_indices = table_data
                    .rows
                    .iter()
                    .enumerate()
                    .filter(|(_, row)| Inspector::row_matches(row, &q_lower))
                    .map(|(index, _)| index)
                    .collect();
            }

            // Update history
            self.last_query = q_lower;
        }

        self.filtered_indices.len()
    }

    // Helper to check if a row matches the query string
    fn row_matches(row: &[Value], q_lower: &str) -> bool {
        for val in row.iter() {
            let text = match val {
                Value::Text(s) => s.to_lowercase(),
                Value::Integer(i) => i.to_string(),
                Value::Real(f) => f.to_string(),
                _ => String::new(),
            };
            if text.contains(q_lower) {
                return true;
            }
        }
        false
    }

    pub fn get_columns(&self) -> Result<String, String> {
        let table_data = self
            .tables
            .get(&self.current_table_name)
            .ok_or_else(|| "No table loaded".to_string())?;

        serde_json::to_string(&table_data.columns).map_err(|e| e.to_string())
    }

    pub fn get_rows_slice(&self, start: usize, count: usize) -> Result<String, String> {
        let table_data = self
            .tables
            .get(&self.current_table_name)
            .ok_or_else(|| "No table loaded".to_string())?;

        let end = (start + count).min(self.filtered_indices.len());
        if start >= self.filtered_indices.len() {
            return Ok("[]".to_string());
        }

        let slice_indices = &self.filtered_indices[start..end];
        let mut json = String::from("[");

        for (i, &original_index) in slice_indices.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }

            json.push('[');

            // Add Row Index (1-based)
            json.push_str(&(original_index + 1).to_string());
            json.push(',');

            let row = &table_data.rows[original_index];

            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    json.push(',');
                }
                match val {
                    Value::Null => json.push_str("null"),
                    Value::Integer(v) => json.push_str(&v.to_string()),
                    Value::Real(v) => json.push_str(&v.to_string()),
                    Value::Text(v) => {
                        let s = serde_json::to_string(v)
                            .unwrap_or_else(|_| "\"<encoding error>\"".to_string());
                        json.push_str(&s);
                    }
                    Value::Blob(_) => json.push_str("\"<blob>\""),
                }
            }
            json.push(']');
        }
        json.push(']');
        Ok(json)
    }
}
