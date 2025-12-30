use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, js_sys};

use super::sidebar::Sidebar;
use super::table_view::TableView;
use crate::core::{TableData, loader};

#[component]
pub fn App() -> impl IntoView {
    let (tables, set_tables) = signal(HashMap::<String, Arc<TableData>>::new());
    let (selected_table_name, set_selected_table_name) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);

    let current_table_data = move || {
        selected_table_name
            .get()
            .and_then(|name| tables.get().get(&name).cloned())
    };

    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                set_is_loading.set(true);

                spawn_local(async move {
                    let promise = file.array_buffer();
                    let future = wasm_bindgen_futures::JsFuture::from(promise);

                    match future.await {
                        Ok(array_buffer) => {
                            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                            let vec = uint8_array.to_vec();

                            match loader::load_tables(vec) {
                                Ok(loaded) => {
                                    let rc_map: HashMap<_, _> =
                                        loaded.into_iter().map(|(k, v)| (k, Arc::new(v))).collect();
                                    set_tables.set(rc_map);
                                }
                                Err(e) => web_sys::console::error_1(&e.into()),
                            }
                        }
                        Err(e) => web_sys::console::error_1(&e),
                    }
                    set_is_loading.set(false);
                });
            }
        }
    };

    view! {
        <div style="display: flex; height: 100vh; overflow: hidden; font-family: sans-serif;">
            <div style="width: 250px; background: #f4f4f4; border-right: 1px solid #ccc; display: flex; flex-direction: column;">
                <div style="padding: 10px; border-bottom: 1px solid #ddd;">
                    <h3>"BA Decoder"</h3>
                    <input type="file" on:change=on_file_change accept=".db,.bytes" />
                    {move || if is_loading.get() { view! { <div>"Loading..."</div> }.into_any() } else { view! {}.into_any() }}
                </div>

                <Sidebar
                    tables=tables.into()
                    selected=selected_table_name.into()
                    on_select=set_selected_table_name
                />
            </div>

            <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden;">
                {move || match current_table_data() {
                    Some(data) => view! { <TableView data=data /> }.into_any(),
                    None => view! { <div style="padding: 20px;">"Select a table to view"</div> }.into_any()
                }}
            </div>
        </div>
    }
}
