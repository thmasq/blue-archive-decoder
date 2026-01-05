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
    let (sidebar_open, set_sidebar_open) = signal(true);

    let current_table_data = move || {
        selected_table_name
            .get()
            .and_then(|name| tables.get().get(&name).cloned())
    };

    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let filename = file.name();
                web_sys::console::log_1(&format!("File selected: {}", filename).into());
                set_is_loading.set(true);

                spawn_local(async move {
                    web_sys::console::log_1(&format!("Starting to read file: {}", filename).into());
                    let promise = file.array_buffer();
                    let future = wasm_bindgen_futures::JsFuture::from(promise);

                    match future.await {
                        Ok(array_buffer) => {
                            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                            let vec = uint8_array.to_vec();

                            web_sys::console::log_1(
                                &format!(
                                    "File read successfully. Size: {} bytes. Beginning parsing...",
                                    vec.len()
                                )
                                .into(),
                            );

                            match loader::load_tables(vec) {
                                Ok(loaded) => {
                                    web_sys::console::log_1(
                                        &format!(
                                            "Successfully parsed and loaded {} tables.",
                                            loaded.len()
                                        )
                                        .into(),
                                    );
                                    let rc_map: HashMap<_, _> =
                                        loaded.into_iter().map(|(k, v)| (k, Arc::new(v))).collect();
                                    set_tables.set(rc_map);
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Error parsing tables: {}", e).into(),
                                    );
                                    web_sys::console::error_1(&e.into());
                                }
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&"Error reading file from input".into());
                            web_sys::console::error_1(&e);
                        }
                    }
                    set_is_loading.set(false);
                });
            }
        }
    };

    view! {
        <div style="display: flex; height: 100vh; overflow: hidden; font-family: sans-serif;">
            <div style=move || format!(
                "width: {}; background: #f4f4f4; border-right: 1px solid #ccc; display: flex; flex-direction: column; transition: width 0.3s ease; overflow: hidden;",
                if sidebar_open.get() { "250px" } else { "0px" }
            )>
                <div style="min-width: 250px; display: flex; flex-direction: column; height: 100%;">
                    <div style="padding: 10px; border-bottom: 1px solid #ddd;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                            <h3 style="margin: 0;">"BA Decoder"</h3>
                            <button
                                on:click=move |_| set_sidebar_open.set(false)
                                style="cursor: pointer; background: none; border: none; font-weight: bold; font-size: 1.2rem; padding: 2px;"
                                title="Collapse Sidebar"
                            >
                                "❮"
                            </button>
                        </div>
                        <input type="file" on:change=on_file_change accept=".db,.bytes" />
                        {move || if is_loading.get() { view! { <div>"Loading..."</div> }.into_any() } else { view! {}.into_any() }}
                    </div>

                    <Sidebar
                        tables=tables.into()
                        selected=selected_table_name.into()
                        on_select=set_selected_table_name
                    />
                </div>
            </div>

            <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative;">
                {move || match current_table_data() {
                    Some(data) => view! {
                        <TableView
                            data=data
                            is_sidebar_open=sidebar_open
                            set_sidebar_open=set_sidebar_open
                        />
                    }.into_any(),
                    None => {
                        // If the sidebar is closed but no table is selected, we still need a way to open it.
                        if !sidebar_open.get() {
                            view! {
                                <div style="padding: 10px; border-bottom: 1px solid #eee; background: #fff; display: flex; align-items: center;">
                                    <button
                                        on:click=move |_| set_sidebar_open.set(true)
                                        style="cursor: pointer; background: none; border: none; font-weight: bold; font-size: 1.2rem; padding: 5px; color: #555; margin-right: 10px;"
                                        title="Expand Sidebar"
                                    >
                                        "❯"
                                    </button>
                                    <span>"Select a table to view"</span>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div style="padding: 20px;">"Select a table to view"</div> }.into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}
