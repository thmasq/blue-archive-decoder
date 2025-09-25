use rusqlite::{Connection, Result as SqliteResult, Transaction};
use std::error::Error;

use crate::localize_excel_generated::*;

#[derive(Debug)]
pub struct DecodedData {
    pub key: i64,
    pub message_id: u32,
    pub english: Option<String>,
    pub chinese: Option<String>,
    pub thai: Option<String>,
    pub japanese: Option<String>,
    pub korean: Option<String>,
}

fn decode_flatbuffer(key: i64, bytes: &[u8]) -> Result<DecodedData, Box<dyn Error>> {
    let localized_msg = root_as_localized_message(bytes)
        .map_err(|e| format!("Failed to parse FlatBuffer: {}", e))?;

    Ok(DecodedData {
        key,
        message_id: localized_msg.message_id(),
        english: localized_msg.english().map(|s| s.to_string()),
        chinese: localized_msg.chinese().map(|s| s.to_string()),
        thai: localized_msg.thai().map(|s| s.to_string()),
        japanese: localized_msg.japanese().map(|s| s.to_string()),
        korean: localized_msg.korean().map(|s| s.to_string()),
    })
}

fn read_database_data(conn: &Connection) -> SqliteResult<Vec<(i64, Vec<u8>)>> {
    let mut stmt = conn.prepare("SELECT Key, Bytes FROM LocalizeDBSchema ORDER BY Key")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?))
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok(data)
}

fn add_language_columns(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "ALTER TABLE LocalizeDBSchema ADD COLUMN message_id INTEGER",
        [],
    )?;
    conn.execute("ALTER TABLE LocalizeDBSchema ADD COLUMN english TEXT", [])?;
    conn.execute("ALTER TABLE LocalizeDBSchema ADD COLUMN chinese TEXT", [])?;
    conn.execute("ALTER TABLE LocalizeDBSchema ADD COLUMN thai TEXT", [])?;
    conn.execute("ALTER TABLE LocalizeDBSchema ADD COLUMN japanese TEXT", [])?;
    conn.execute("ALTER TABLE LocalizeDBSchema ADD COLUMN korean TEXT", [])?;
    Ok(())
}

fn update_row_with_decoded_data(tx: &Transaction, decoded: &DecodedData) -> SqliteResult<()> {
    tx.execute(
        "UPDATE LocalizeDBSchema SET message_id = ?1, english = ?2, chinese = ?3, thai = ?4, japanese = ?5, korean = ?6 WHERE Key = ?7",
        [
            Some(decoded.message_id.to_string()).as_deref(),
            decoded.english.as_deref(),
            decoded.chinese.as_deref(),
            decoded.thai.as_deref(),
            decoded.japanese.as_deref(),
            decoded.korean.as_deref(),
            Some(&decoded.key.to_string()).as_deref().map(|x| x.as_str()),
        ],
    )?;
    Ok(())
}

fn drop_bytes_column(conn: &Connection) -> SqliteResult<()> {
    conn.execute("ALTER TABLE LocalizeDBSchema DROP COLUMN Bytes", [])?;
    Ok(())
}

/// Process the LocalizeDBSchema table by decoding FlatBuffer blobs into language columns
pub fn process_localize_db_schema(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    println!("Processing LocalizeDBSchema table...");

    println!("Reading existing data...");
    let db_data = read_database_data(conn)?;
    println!("Found {} records in LocalizeDBSchema", db_data.len());

    if db_data.is_empty() {
        println!("No data found in LocalizeDBSchema table, skipping...");
        return Ok(());
    }

    println!("Adding language columns...");
    add_language_columns(conn)?;

    println!("Decoding FlatBuffer data...");
    let mut successful_decodes = 0;
    let mut failed_decodes = 0;

    const BATCH_SIZE: usize = 100;
    for batch in db_data.chunks(BATCH_SIZE) {
        let tx = conn.transaction()?;

        for (key, bytes) in batch {
            if bytes.is_empty() {
                println!("Warning: Key {} has empty bytes, skipping", key);
                failed_decodes += 1;
                continue;
            }

            match decode_flatbuffer(*key, bytes) {
                Ok(decoded_data) => {
                    update_row_with_decoded_data(&tx, &decoded_data)?;
                    successful_decodes += 1;

                    if successful_decodes <= 3 {
                        println!("Successfully decoded key {}: {:?}", key, decoded_data);
                    }
                }
                Err(e) => {
                    println!("Warning: Failed to decode key {}: {}", key, e);
                    failed_decodes += 1;
                }
            }
        }

        tx.commit()?;

        if successful_decodes % 100 == 0 && successful_decodes > 0 {
            println!("Processed {} records...", successful_decodes);
        }
    }

    println!(
        "LocalizeDBSchema decoding complete: {} successful, {} failed",
        successful_decodes, failed_decodes
    );

    if successful_decodes == 0 {
        eprintln!("Error: No data was successfully decoded from LocalizeDBSchema");
        return Err("No data was successfully decoded".into());
    }

    println!("Dropping original Bytes column...");
    drop_bytes_column(conn)?;

    println!(
        "Successfully processed {} LocalizeDBSchema records",
        successful_decodes
    );

    Ok(())
}
