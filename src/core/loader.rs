use super::TableData;
use crate::db_migrator;
use crate::utils::xxhash32;
use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;

/// Loads all tables from a `SQLite` database binary and converts them into [`TableData`] structures.
///
/// This function initializes an in-memory `SQLite` database from the provided file bytes,
/// applies any registered custom loaders, then iterates over the remaining tables found
/// in the database schema. After loading, it performs additional post-processing to enrich
/// certain tables (such as resolving speaker names in scenario scripts).
///
/// # Errors
///
/// Returns an `Err(String)` if the `SQLite` database cannot be opened or initialized from
/// the provided `file_data`, or if the list of tables cannot be retrieved from the database
/// schema. The error string contains a human-readable description of the failure.
///
/// Errors originating from individual table queries or custom loaders are logged and
/// skipped, and do **not** cause this function to return an error.
pub fn load_tables(file_data: Vec<u8>) -> Result<HashMap<String, TableData>, String> {
    web_sys::console::log_1(&"loader::load_tables called.".into());

    let cursor = Cursor::new(file_data);
    let mut db = Database::new(cursor).map_err(|e| format!("Failed to open DB: {e}"))?;

    web_sys::console::log_1(&"Database initialized successfully.".into());

    let mut tables = HashMap::new();

    let mut loaders = HashMap::new();
    db_migrator::register_loaders(&mut loaders);

    web_sys::console::log_1(&format!("Registered {} custom loaders.", loaders.len()).into());

    for (name, loader) in loaders {
        if let Ok((cols, rows)) = loader(&mut db) {
            web_sys::console::log_1(&format!("Loaded custom table: {name}").into());
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

    enrich_scenario_script(&mut tables);

    web_sys::console::log_1(
        &format!("Loader finished. Total loaded tables: {}", tables.len()).into(),
    );

    Ok(tables)
}

fn enrich_scenario_script(tables: &mut HashMap<String, TableData>) {
    let Some(name_table) = tables.get("ScenarioCharacterNameExcel") else {
        return;
    };

    let id_col_idx = name_table
        .columns
        .iter()
        .position(|c| c == "character_name");
    let en_col_idx = name_table.columns.iter().position(|c| c == "name_en");

    let mut name_map: HashMap<i64, String> = HashMap::new();

    if let (Some(id_idx), Some(en_idx)) = (id_col_idx, en_col_idx) {
        for row in &name_table.rows {
            let id = match row.get(id_idx) {
                Some(Value::Integer(val)) => *val,
                Some(Value::Real(val)) => *val as i64,
                _ => continue,
            };

            let name = match row.get(en_idx) {
                Some(Value::Text(s)) => s.clone(),
                _ => continue,
            };

            name_map.insert(id, name);
        }
    }

    if let Some(script_table) = tables.get_mut("ScenarioScriptExcel") {
        let script_col_idx = script_table
            .columns
            .iter()
            .position(|c| c == "script_kr" || c == "Text" || c == "Script");

        if let Some(col_idx) = script_col_idx {
            script_table.columns.insert(0, "Speaker".to_string());

            for row in &mut script_table.rows {
                let mut decoded_name = Value::Null;

                if let Some(Value::Text(script_text)) = row.get(col_idx) {
                    let parts: Vec<&str> = script_text.split(';').collect();

                    if parts.len() >= 2 && !script_text.trim().starts_with('#') {
                        let name_part = parts[1];

                        let hash_u32 = xxhash32(name_part.as_bytes(), 0);
                        let hash_i64 = i64::from(hash_u32);

                        if let Some(en_name) = name_map.get(&hash_i64) {
                            decoded_name = Value::Text(en_name.clone());
                        } else {
                            decoded_name = Value::Text(format!("{name_part} (?)"));
                        }
                    }
                }

                row.insert(0, decoded_name);
            }
        }
    }
}
