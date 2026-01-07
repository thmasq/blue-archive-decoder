use leptos::ev;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_event_listener;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, js_sys};

use super::sheet_tabs::SheetTabs;
use super::table_selector_modal::TableSelectorModal;
use super::table_view::TableView;
use crate::core::{TableData, loader};

#[component]
pub fn App() -> impl IntoView {
    let (tables, set_tables) = signal(HashMap::<String, Arc<TableData>>::new());
    let (selected_table_name, set_selected_table_name) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);
    let (show_modal, set_show_modal) = signal(false);

    let current_table_data = move || {
        selected_table_name
            .get()
            .and_then(|name| tables.get().get(&name).cloned())
    };

    let _ = use_event_listener(window(), ev::keydown, move |ev: ev::KeyboardEvent| {
        if ev.ctrl_key() || ev.alt_key() || ev.meta_key() {
            return;
        }

        if let Some(target) = ev.target() {
            if let Some(el) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                let t = el.type_();
                if t == "text" || t == "search" || t == "password" {
                    return;
                }
            }
        }

        if ev.key() == "t" {
            ev.prevent_default();
            set_show_modal.set(true);
        }
    });

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
                            if let Ok(loaded) = loader::load_tables(vec) {
                                let rc_map: HashMap<_, _> =
                                    loaded.into_iter().map(|(k, v)| (k, Arc::new(v))).collect();
                                set_tables.set(rc_map);
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
        <div style="display: flex; flex-direction: column; height: 100vh; overflow: hidden; font-family: 'Roboto', 'Noto Sans', sans-serif; background: #f9fbfd;">

            <div style="background: #ffffff; border-bottom: 1px solid #dadce0; padding: 8px 16px; display: flex; align-items: center; justify-content: space-between; height: 48px; box-sizing: border-box;">
                <div style="display: flex; align-items: center; gap: 16px;">
                    <h3 style="margin: 0; color: #444; font-size: 1.1rem; font-weight: 500;">"BA Decoder"</h3>
                </div>

                <div style="display: flex; align-items: center; gap: 10px;">
                    {move || if is_loading.get() {
                        view! { <span style="font-size: 0.85rem; color: #666;">"Processing..."</span> }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                    <label style="cursor: pointer; background: #f1f3f4; padding: 6px 12px; border-radius: 4px; font-size: 0.85rem; font-weight: 500; color: #333; transition: background 0.2s;">
                        "Open File"
                        <input type="file" on:change=on_file_change accept=".db,.bytes" style="display: none;" />
                    </label>
                </div>
            </div>

            <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative; background: #fff;">
                {move || match current_table_data() {
                    Some(data) => view! { <TableView data=data /> }.into_any(),
                    None => view! {
                        <div style="display: flex; justify-content: center; align-items: center; height: 100%; color: #888;">
                            <div style="text-align: center;">
                                <p style="font-size: 1.5rem; margin-bottom: 10px;">"No Table Selected"</p>
                                <p>"Upload a file and select a table from the tabs below."</p>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>

            <SheetTabs
                tables=tables.into()
                selected=selected_table_name.into()
                on_select=set_selected_table_name
                on_add=move |_| set_show_modal.set(true)
            />

            <TableSelectorModal
                is_open=show_modal
                set_open=set_show_modal
                tables=tables.into()
                on_select=move |name| {
                    set_selected_table_name.set(Some(name));
                }
            />
        </div>
    }
}
