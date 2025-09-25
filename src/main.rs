use clap::Parser;
use rusqlite::Connection;
use std::error::Error;
use std::path::Path;

mod localize_excel;
mod localize_excel_generated;

use localize_excel::process_localize_db_schema;

#[derive(Parser)]
#[command(name = "localize-db-modifier")]
#[command(
    about = "Modifies Blue Archive localization database by decoding FlatBuffer blobs into language columns"
)]
struct Args {
    database: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !Path::new(&args.database).exists() {
        eprintln!("Error: Database file '{}' does not exist", args.database);
        std::process::exit(1);
    }

    println!("Processing database: {}", args.database);

    let mut conn = Connection::open(&args.database)?;

    // Process LocalizeDBSchema table
    process_localize_db_schema(&mut conn)?;

    println!("Database modification complete!");

    Ok(())
}
