extern crate alloc;

mod blue_archive_generated;
mod db_migrator;

use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// Use the registry type from db_migrator
use db_migrator::TableLoader;

#[wasm_bindgen]
pub struct Inspector {
    // The DB connection (wrapped cursor)
    db: Database<Cursor<Vec<u8>>>,

    // Registry of "Virtual" tables that need decoding
    loaders: HashMap<String, TableLoader>,

    // The master copy of data for the currently selected table
    cached_rows: Vec<Vec<Value>>,
    cached_columns: Vec<String>,

    // Pointers to rows matching the current filter
    filtered_indices: Vec<usize>,
}

#[wasm_bindgen]
impl Inspector {
    #[wasm_bindgen(constructor)]
    pub fn new(file_data: Vec<u8>) -> Result<Inspector, String> {
        let cursor = Cursor::new(file_data);
        let db = Database::new(cursor).map_err(|e| format!("Failed to open DB: {}", e))?;

        // Initialize the inspector
        let mut inspector = Inspector {
            db,
            loaders: HashMap::new(),
            cached_rows: Vec::new(),
            cached_columns: Vec::new(),
            filtered_indices: Vec::new(),
        };

        // Register all Flatbuffer loaders
        db_migrator::register_loaders(&mut inspector.loaders);

        Ok(inspector)
    }

    /// Returns a list of all available tables (Physical SQL tables + Virtual Flatbuffer tables)
    pub fn get_tables(&mut self) -> Result<Box<[JsValue]>, String> {
        // 1. Get real SQL tables
        let mut table_names: Vec<String> = self
            .db
            .tables()
            .map_err(|e| format!("Failed to list tables: {}", e))?;

        // 2. Add virtual tables from loaders
        for name in self.loaders.keys() {
            if !table_names.contains(name) {
                table_names.push(name.clone());
            }
        }

        table_names.sort();

        let js_array: Vec<JsValue> = table_names.into_iter().map(JsValue::from).collect();
        Ok(js_array.into_boxed_slice())
    }

    /// Loads a table into memory.
    /// If it's a virtual table, it triggers the decoding logic.
    /// If it's a real table, it runs a SELECT * query.
    pub fn load_table(&mut self, table_name: &str) -> Result<usize, String> {
        // Clear previous data
        self.cached_rows.clear();
        self.cached_columns.clear();
        self.filtered_indices.clear();

        // Check if it is a virtual table (Flatbuffer)
        if let Some(loader) = self.loaders.get(table_name) {
            // Execute the closure to get data
            let (cols, rows) = loader(&mut self.db)
                .map_err(|e| format!("Failed to load virtual table {}: {}", table_name, e))?;

            self.cached_columns = cols;
            self.cached_rows = rows;
        } else {
            // Fallback: Standard SQL table
            self.cached_columns = self
                .db
                .get_table_columns(table_name)
                .map_err(|e| format!("Failed to get columns: {}", e))?;

            let query = SelectQuery::new(table_name);
            let raw_rows = self
                .db
                .execute_query(&query)
                .map_err(|e| format!("Failed to query data: {}", e))?;

            // Convert SQL Rows to Vec<Value> for uniform storage
            self.cached_rows = raw_rows
                .into_iter()
                .map(|row| row.values().into_iter().cloned().collect())
                .collect();
        }

        // Initialize view with all rows
        self.filtered_indices = (0..self.cached_rows.len()).collect();

        Ok(self.filtered_indices.len())
    }

    /// Simple string search filter across all columns
    pub fn apply_filter(&mut self, query: &str) -> usize {
        if query.is_empty() {
            if self.filtered_indices.len() != self.cached_rows.len() {
                self.filtered_indices = (0..self.cached_rows.len()).collect();
            }
        } else {
            let q_lower = query.to_lowercase();
            self.filtered_indices = self
                .cached_rows
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    // FIXED: row is &&Vec<Value> here. We must call .iter() or dereference it.
                    for val in row.iter() {
                        let text = match val {
                            Value::Text(s) => s.to_lowercase(),
                            Value::Integer(i) => i.to_string(),
                            Value::Real(f) => f.to_string(),
                            _ => String::new(),
                        };
                        if text.contains(&q_lower) {
                            return true;
                        }
                    }
                    false
                })
                .map(|(index, _)| index)
                .collect();
        }

        self.filtered_indices.len()
    }

    pub fn get_columns(&self) -> Result<String, String> {
        let mut json = String::from("[");
        for (i, col) in self.cached_columns.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!("{:?}", col));
        }
        json.push(']');
        Ok(json)
    }

    pub fn get_rows_slice(&self, start: usize, count: usize) -> Result<String, String> {
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

            let row = &self.cached_rows[original_index];

            for (j, val) in row.iter().enumerate() {
                if j > 0 {
                    json.push(',');
                }
                match val {
                    Value::Null => json.push_str("null"),
                    Value::Integer(v) => json.push_str(&v.to_string()),
                    Value::Real(v) => json.push_str(&v.to_string()),
                    Value::Text(v) => json.push_str(&format!("{:?}", v)),
                    Value::Blob(_) => json.push_str("\"<blob>\""),
                }
            }
            json.push(']');
        }
        json.push(']');
        Ok(json)
    }
}
