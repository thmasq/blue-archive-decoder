use crate::core::TableData;
use crate::utils::{self, create_key, xor_bytes};
use sqlite_wasm_reader::Value;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;

type ZipEntryLoader =
    Box<dyn Fn(&[u8]) -> Result<(Vec<String>, Vec<Vec<Value>>), Box<dyn std::error::Error>>>;

fn generate_zip_password(filename: &str) -> String {
    let key = create_key(filename, 15);
    utils::base64_encode(&key)
}

fn json_to_sqlite_value(val: &serde_json::Value) -> Value {
    match val {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Integer(i64::from(*b)),
        serde_json::Value::Number(n) => n.as_i64().map_or_else(
            || {
                n.as_f64()
                    .map_or_else(|| Value::Text(n.to_string()), Value::Real)
            },
            Value::Integer,
        ),
        serde_json::Value::String(s) => Value::Text(s.clone()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => Value::Text(val.to_string()),
    }
}

macro_rules! register_zip_table {
    ($registry:expr, $base_name:ident) => {
        paste::paste! {
            let loader = Box::new(move |bytes: &[u8]| {
                let mut data = bytes.to_vec();
                let seed_name = stringify!([< $base_name Table >]);
                let key = create_key(seed_name, data.len());
                xor_bytes(&mut data, &key);

                type RootItem<'a> = crate::blue_archive_generated::global::[< $base_name Table >]<'a>;
                let root = flatbuffers::root::<RootItem>(&data)?;

                let json_val = serde_json::to_value(&root)?;

                let item_seed_name = stringify!($base_name)
                    .strip_suffix("Excel")
                    .unwrap_or(stringify!($base_name));
                let item_key = create_key(item_seed_name, 8);

                let mut result_rows = Vec::new();
                let mut headers = Vec::new();

                if let serde_json::Value::Object(map) = json_val {
                    if let Some(serde_json::Value::Array(list)) = map.get("data_list") {
                         if let Some(serde_json::Value::Object(first_item)) = list.first() {
                             headers = first_item.keys()
                                .filter(|k| !k.ends_with("_th") && !k.ends_with("_tw") && *k != "tw" && *k != "th")
                                .cloned()
                                .collect();
                         }

                     for item in list {
                        if let serde_json::Value::Object(item_map) = item {
                            let mut row_vec = Vec::new();
                            for header in &headers {
                                let val = item_map.get(header).unwrap_or(&serde_json::Value::Null);

                                let processed_val = match val {
                                    serde_json::Value::String(s) => {
                                        let decrypted = utils::decrypt_string(s, &item_key);
                                        serde_json::Value::String(decrypted)
                                    },
                                    serde_json::Value::Number(n) => {
                                        if let Some(i) = n.as_i64() {
                                            let d = utils::decrypt_i64(i, &item_key);
                                            serde_json::Value::Number(serde_json::Number::from(d))
                                        } else {
                                            // Floating point or u64 > i64::MAX?
                                            // Currently leaving as-is or could try u64 decryption if needed.
                                            // For now, assuming standard integer IDs.
                                            val.clone()
                                        }
                                    },
                                    _ => val.clone(),
                                };

                                row_vec.push(json_to_sqlite_value(&processed_val));
                            }
                            result_rows.push(row_vec);
                        }
                     }
                    }
                }

                Ok((headers, result_rows))
            });

            let file_name = format!("{}.bytes", stringify!([< $base_name Table >]).to_lowercase());
            let table_name = stringify!($base_name).to_string();

            $registry.insert(file_name, (table_name, loader));
        }
    };
}

pub fn load_zip_tables(zip_data: Vec<u8>) -> Result<HashMap<String, TableData>, String> {
    web_sys::console::log_1(&"zip_reader::load_zip_tables called.".into());

    let reader = Cursor::new(zip_data);
    let password = generate_zip_password("Excel.zip");

    web_sys::console::log_1(&format!("Generated ZIP password: {}", password).into());

    let mut zip =
        ZipArchive::new(reader).map_err(|e| format!("Failed to read ZIP structure: {e}"))?;

    let mut registry: HashMap<String, (String, ZipEntryLoader)> = HashMap::new();

    register_zip_table!(registry, LocalizeCharProfileExcel);

    let mut tables = HashMap::new();

    for i in 0..zip.len() {
        let file_result = zip.by_index_decrypt(i, password.as_bytes());

        match file_result {
            Ok(mut file) => {
                let file_name = file.name().to_lowercase();
                if let Some((_, (table_name, loader))) =
                    registry.iter().find(|(k, _)| file_name.ends_with(*k))
                {
                    let mut buffer = Vec::new();
                    if file.read_to_end(&mut buffer).is_ok() {
                        match loader(&buffer) {
                            Ok((cols, rows)) => {
                                tables.insert(
                                    table_name.clone(),
                                    TableData {
                                        name: table_name.clone(),
                                        columns: cols,
                                        rows,
                                    },
                                );
                            }
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("Failed to decode {}: {:?}", file_name, e).into(),
                                );
                            }
                        }
                    }
                }
            }
            Err(zip::result::ZipError::InvalidPassword) => {
                web_sys::console::error_1(
                    &format!("Password incorrect for file index {}", i).into(),
                );
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Zip error at index {}: {:?}", i, e).into());
            }
        }
    }

    Ok(tables)
}
