extern crate alloc;

mod blue_archive_generated;
mod db_migrator;

use regex::Regex;
use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Write;
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

    #[wasm_bindgen(skip)]
    cached_regex: Option<Regex>,
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
            if name.ends_with("DBSchema") {
                continue;
            }

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
            cached_regex: None,
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
    pub fn load_table(&mut self, table_name: &str) -> Result<usize, String> {
        if !self.tables.contains_key(table_name) {
            return Err(format!("Table not found: {}", table_name));
        }

        self.current_table_name = table_name.to_string();

        let row_count = self.tables.get(table_name).unwrap().rows.len();
        self.filtered_indices = (0..row_count).collect();

        self.last_query.clear();
        self.cached_regex = None;

        Ok(row_count)
    }

    /// String search
    pub fn apply_filter(&mut self, query: &str) -> usize {
        let table_data = match self.tables.get(&self.current_table_name) {
            Some(t) => t,
            None => return 0,
        };

        if query.is_empty() {
            if self.filtered_indices.len() != table_data.rows.len() {
                self.filtered_indices = (0..table_data.rows.len()).collect();
            }
            self.last_query.clear();
            self.cached_regex = None;
            return self.filtered_indices.len();
        }

        // Initialize or Update Regex
        if self.cached_regex.is_none() || query != self.last_query {
            let re = regex::RegexBuilder::new(query)
                .case_insensitive(true)
                .build()
                .or_else(|_| {
                    regex::RegexBuilder::new(&regex::escape(query))
                        .case_insensitive(true)
                        .build()
                })
                .ok();

            self.cached_regex = re;
            self.last_query = query.to_string();
        }

        let re = match &self.cached_regex {
            Some(r) => r,
            None => return 0,
        };

        self.filtered_indices = table_data
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| Inspector::row_matches(row, re))
            .map(|(index, _)| index)
            .collect();

        self.filtered_indices.len()
    }

    fn row_matches(row: &[Value], re: &Regex) -> bool {
        for val in row.iter() {
            let matches = match val {
                Value::Text(s) => re.is_match(s),
                Value::Integer(i) => {
                    let mut buf = [0u8; 32];
                    let mut cursor = Cursor::new(&mut buf[..]);
                    let _ = write!(cursor, "{}", i);
                    let len = cursor.position() as usize;
                    // Integers are always valid UTF-8
                    let s = unsafe { std::str::from_utf8_unchecked(&buf[..len]) };
                    re.is_match(s)
                }
                Value::Real(f) => {
                    let mut buf = [0u8; 64];
                    let mut cursor = Cursor::new(&mut buf[..]);
                    let _ = write!(cursor, "{}", f);
                    let len = cursor.position() as usize;
                    let s = unsafe { std::str::from_utf8_unchecked(&buf[..len]) };
                    re.is_match(s)
                }
                _ => false,
            };

            if matches {
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
