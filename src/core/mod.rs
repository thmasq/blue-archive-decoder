pub mod loader;
pub mod text_measurer;

use sqlite_wasm_reader::Value;

pub const FONT_SIZE: f32 = 13.0;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableData {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}
