pub mod loader;
pub mod text_measurer;

use sqlite_wasm_reader::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct TableData {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}
