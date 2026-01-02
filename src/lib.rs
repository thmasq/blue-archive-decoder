extern crate alloc;

pub mod blue_archive_generated;
pub mod components;
pub mod core;
pub mod db_migrator;

use crate::components::app::App;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(move || view! { <App /> })
}
