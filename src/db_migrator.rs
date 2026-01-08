#![allow(clippy::items_after_statements)]

use crate::blue_archive_generated::global::{
    AcademyMessangerExcel, CharacterDialogEmojiExcel, CharacterDialogEventExcel,
    CharacterDialogExcel, CharacterDialogSubtitleExcel, CharacterVoiceSubtitleExcel,
    LocalizeErrorExcel, LocalizeEtcExcel, LocalizeExcel, LocalizeSkillExcel,
    ScenarioCharacterEmotionExcel, ScenarioCharacterNameExcel, ScenarioScriptExcel,
    TutorialCharacterDialogExcel,
};
use sqlite_wasm_reader::{Database, SelectQuery, Value};
use std::collections::HashMap;
use std::io::Cursor;

// Define the signature for a loader function.
// It takes a reference to the database and returns a tuple of (Column Names, Rows).
pub type TableLoader = Box<
    dyn Fn(
        &mut Database<Cursor<Vec<u8>>>,
    ) -> Result<(Vec<String>, Vec<Vec<Value>>), Box<dyn std::error::Error>>,
>;

/// Converts a `serde_json::Value` to a `sqlite_wasm_reader::Value`
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
        // Serialize Arrays and Objects back to JSON strings for storage/display
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => Value::Text(val.to_string()),
    }
}

macro_rules! register_table {
    // Pattern 1: Standard naming (recurses to Pattern 2)
    ($registry:expr, $base_name:ident) => {
        paste::paste! {
            register_table!($registry, $base_name, [< $base_name Excel >], [< $base_name ExcelT >])
        }
    };

    // Pattern 2: Explicit naming
    // Note: $native_type is unused here as we serialize the flatbuffer root ($root_type) directly
    ($registry:expr, $base_name:ident, $root_type:ident, $native_type:ident) => {
        paste::paste! {
            {
                let source_table = stringify!([< $base_name DBSchema >]);
                let target_table = stringify!([< $base_name Excel >]); // The name shown in the viewer

                // Create the loader closure
                let loader = Box::new(move |db: &mut Database<Cursor<Vec<u8>>>| {
                    // 1. Get the bytes from ALL rows
                    let query_str = format!("SELECT Bytes FROM {}", source_table);
                    let query = SelectQuery::parse(&query_str).map_err(|e| format!("Query parse error: {:?}", e))?;

                    let rows = db.execute_query(&query).map_err(|e| format!("DB exec error: {:?}", e))?;

                    let mut result_rows = Vec::new();
                    let mut headers = Vec::new();
                    let mut first = true;

                    // 2. Iterate over every row in the DB table
                    for row in rows {
                         let bytes = match row.values().next() {
                            Some(Value::Blob(b)) => b.clone(),
                            Some(Value::Text(s)) => s.as_bytes().to_vec(),
                            _ => continue, // Skip rows with no valid blob
                        };

                        if bytes.is_empty() {
                            continue;
                        }

                        // 3. Decode Flatbuffer Item directly
                        // We use the root type ($root_type / SomethingExcel) which implements Serialize
                        type RootItem<'a> = $root_type<'a>;

                        // Note: flatbuffers::root verifies the buffer.
                        let root = flatbuffers::root::<RootItem>(&bytes)?;

                        // 4. Serialize to JSON
                        // We rely on the flatbuffer struct implementing Serialize (via --rust-serialize)
                        let json_val = serde_json::to_value(&root)?;

                        if let serde_json::Value::Object(map) = json_val {
                            if first {
                                headers = map.keys()
                                    .filter(|k| !k.ends_with("_th") && !k.ends_with("_tw") && *k != "tw" && *k != "th")
                                    .cloned()
                                    .collect();
                                first = false;
                            }

                            // Ensure values follow header order
                            let mut row_vec = Vec::new();
                            for header in &headers {
                                let val = map.get(header).unwrap_or(&serde_json::Value::Null);
                                row_vec.push(json_to_sqlite_value(val));
                            }

                            result_rows.push(row_vec);
                        }
                    }

                    Ok((headers, result_rows))
                });

                $registry.insert(target_table.to_string(), loader);
            }
        }
    };
}

pub fn register_loaders<S: ::std::hash::BuildHasher>(
    registry: &mut HashMap<String, TableLoader, S>,
) {
    register_table!(registry, AcademyMessanger);
    register_table!(registry, CharacterDialog);
    register_table!(registry, CharacterDialogEmoji);
    register_table!(registry, CharacterDialogEvent);
    register_table!(registry, CharacterDialogSubtitle);
    register_table!(registry, CharacterVoiceSubtitle);
    register_table!(registry, Localize);
    register_table!(registry, LocalizeError);
    register_table!(registry, LocalizeEtc);
    register_table!(registry, LocalizeSkill);
    register_table!(registry, ScenarioCharacterEmotion);
    register_table!(registry, ScenarioCharacterName);
    register_table!(registry, ScenarioScript);
    register_table!(registry, TutorialCharacterDialog);
}
