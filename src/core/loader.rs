use super::TableData;
use crate::db_migrator;
use sqlite_wasm_reader::{Database, SelectQuery};
use std::collections::HashMap;
use std::io::Cursor;

pub fn load_tables(file_data: Vec<u8>) -> Result<HashMap<String, TableData>, String> {
    let cursor = Cursor::new(file_data);
    let mut db = Database::new(cursor).map_err(|e| format!("Failed to open DB: {}", e))?;

    let mut tables = HashMap::new();

    let mut loaders = HashMap::new();
    db_migrator::register_loaders(&mut loaders);

    for (name, loader) in loaders {
        if let Ok((cols, rows)) = loader(&mut db) {
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

    Ok(tables)
}
