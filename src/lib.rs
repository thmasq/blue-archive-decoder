#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]

extern crate alloc;

pub mod blue_archive_generated;
pub mod components;
pub mod core;
pub mod db_migrator;
pub mod utils;
pub mod zip_reader;

use crate::components::app::App;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(move || view! { <App /> });
}
