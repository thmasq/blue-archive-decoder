extern crate alloc;

use sqlite_wasm_reader::{Database, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};

// Declare the modules so they are compiled into the binary.
// This assumes these files exist in src/
mod blue_archive_generated;
mod db_migrator;

use db_migrator::TableLoader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup: Define the database file path
    // You can change this to match your local file path
    let db_path = "ExcelDB.db";

    println!("--- Blue Archive DB Inspector (CLI) ---");
    println!("Attempting to open: {}", db_path);

    // 2. Load the file into memory
    let mut file = File::open(db_path).map_err(|e| {
        format!(
            "Failed to open '{}'. Make sure the DB file is in the current directory.\nError: {}",
            db_path, e
        )
    })?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("File loaded. Size: {} bytes.", buffer.len());

    // 3. Initialize the sqlite_wasm_reader Database
    let cursor = Cursor::new(buffer);
    let mut db = Database::new(cursor).map_err(|e| format!("Failed to parse SQLite DB: {}", e))?;

    // 4. Register the loaders (The logic from db_migrator.rs)
    let mut loaders: HashMap<String, TableLoader> = HashMap::new();
    db_migrator::register_loaders(&mut loaders);

    println!(
        "Registry loaded. Available virtual tables: {}",
        loaders.len()
    );

    // 5. Target specific table
    let target_table = "LocalizeExcel";
    println!("\n--- Querying Virtual Table: {} ---", target_table);

    if let Some(loader) = loaders.get(target_table) {
        // Execute the loader logic
        // This runs the SELECT bytes, Flatbuffer decode, and Debug parse
        match loader(&mut db) {
            Ok((columns, rows)) => {
                println!("Success! Decoded {} rows.", rows.len());
                println!("Columns ({}): {:?}", columns.len(), columns);

                // Print the first few rows to verify data
                let preview_count = 5;
                println!("\nPreviewing first {} rows:", preview_count);

                for (i, row) in rows.iter().take(preview_count).enumerate() {
                    println!("\n[Row {}]", i + 1);
                    for (col_idx, val) in row.iter().enumerate() {
                        let col_name = columns.get(col_idx).map(|s| s.as_str()).unwrap_or("???");
                        let val_display = match val {
                            Value::Text(t) => format!("\"{}\"", t),
                            Value::Integer(n) => format!("{}", n),
                            Value::Real(f) => format!("{}", f),
                            Value::Blob(b) => format!("<Blob {} bytes>", b.len()),
                            Value::Null => "NULL".to_string(),
                        };
                        println!("  {}: {}", col_name, val_display);
                    }
                }
            }
            Err(e) => eprintln!("Error loading table: {}", e),
        }
    } else {
        eprintln!("Table '{}' not found in registry.", target_table);
        eprintln!("Available tables: {:?}", loaders.keys().collect::<Vec<_>>());
    }

    Ok(())
}
