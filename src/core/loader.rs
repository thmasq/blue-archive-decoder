use super::TableData;
use crate::db_migrator;
use sqlite_wasm_reader::{Database, SelectQuery};
use std::collections::HashMap;
use std::io::Cursor;

pub fn load_tables(file_data: Vec<u8>) -> Result<HashMap<String, TableData>, String> {
    web_sys::console::log_1(&"loader::load_tables called.".into());

    let cursor = Cursor::new(file_data);
    let mut db = Database::new(cursor).map_err(|e| format!("Failed to open DB: {}", e))?;

    web_sys::console::log_1(&"Database initialized successfully.".into());

    let mut tables = HashMap::new();

    let mut loaders = HashMap::new();
    db_migrator::register_loaders(&mut loaders);

    web_sys::console::log_1(&format!("Registered {} custom loaders.", loaders.len()).into());

    for (name, loader) in loaders {
        if let Ok((cols, rows)) = loader(&mut db) {
            web_sys::console::log_1(&format!("Loaded custom table: {}", name).into());
            tables.insert(
                name.clone(),
                TableData {
                    name,
                    columns: cols,
                    rows,
                },
            );
        }
    }

    let sql_table_names = db.tables().map_err(|e| e.to_string())?;
    web_sys::console::log_1(
        &format!(
            "Found {} total tables in SQLite schema.",
            sql_table_names.len()
        )
        .into(),
    );

    for name in sql_table_names {
        if name.ends_with("DBSchema") || tables.contains_key(&name) {
            continue;
        }

        let columns = db.get_table_columns(&name).unwrap_or_default();
        let query = SelectQuery::new(&name);

        if let Ok(raw_rows) = db.execute_query(&query) {
            let rows = raw_rows
                .into_iter()
                .map(|r| r.values().cloned().collect())
                .collect();

            tables.insert(
                name.clone(),
                TableData {
                    name: name.clone(),
                    columns,
                    rows,
                },
            );
        }
    }

    web_sys::console::log_1(
        &format!("Loader finished. Total loaded tables: {}", tables.len()).into(),
    );

    Ok(tables)
}
