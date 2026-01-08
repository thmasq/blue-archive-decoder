use crate::core::TableData;
use leptos::ev;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[must_use]
#[component]
#[allow(clippy::implicit_hasher)]
pub fn TableSelectorModal(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] set_open: WriteSignal<bool>,
    tables: Signal<HashMap<String, Arc<TableData>>>,
    #[prop(into)] on_select: Callback<String>,
) -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let input_ref = NodeRef::<leptos::html::Input>::new();

    Effect::new(move |_| {
        if is_open.get() {
            set_search.set(String::new());
            request_animation_frame(move || {
                if let Some(el) = input_ref.get() {
                    let _ = el.focus();
                }
            });
        }
    });

    let filtered_tables = move || {
        let query = search.get().to_lowercase();
        let mut names: Vec<String> = tables.get().keys().cloned().collect();
        names.sort();

        if query.is_empty() {
            names
        } else {
            names
                .into_iter()
                .filter(|n| n.to_lowercase().contains(&query))
                .collect()
        }
    };

    view! {
        <Show when=move || is_open.get()>
            <div
                style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center; font-family: sans-serif;"
                on:click=move |_| set_open.set(false)
            >
                <div
                    style="background: white; width: 500px; max-width: 90%; max-height: 80vh; border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); display: flex; flex-direction: column; overflow: hidden;"
                    on:click=move |ev| ev.stop_propagation()
                >
                    <div style="padding: 16px; border-bottom: 1px solid #eee; background: #f8f9fa;">
                        <h3 style="margin: 0 0 12px 0; color: #333; font-size: 1.1rem;">"Go to Table"</h3>
                        <input
                            node_ref=input_ref
                            type="text"
                            placeholder="Type to search..."
                            prop:value=move || search.get()
                            on:input=move |ev| set_search.set(event_target_value(&ev))
                            style="width: 100%; padding: 10px; font-size: 15px; border: 1px solid #dadce0; border-radius: 4px; box-sizing: border-box; outline: none;"
                            on:keydown=move |ev: ev::KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    if let Some(first) = filtered_tables().first() {
                                        on_select.run(first.clone());
                                        set_open.set(false);
                                    }
                                } else if ev.key() == "Escape" {
                                    set_open.set(false);
                                }
                            }
                        />
                    </div>
                    <div style="flex: 1; overflow-y: auto; padding: 0;">
                        <ul style="list-style: none; margin: 0; padding: 0;">
                            <For
                                each=filtered_tables
                                key=|name| name.clone()
                                children=move |name| {
                                    let n_clone = name.clone();
                                    view! {
                                        <li
                                            style="padding: 12px 16px; border-bottom: 1px solid #f1f3f4; cursor: pointer; transition: background 0.1s; color: #333;"
                                            on:click=move |_| {
                                                on_select.run(n_clone.clone());
                                                set_open.set(false);
                                            }
                                            on:mouseover=move |ev: ev::MouseEvent| {
                                                 let target: leptos::web_sys::HtmlElement = event_target(&ev);
                                                 let _ = target.style().set_property("background-color", "#e8f0fe");
                                            }
                                            on:mouseout=move |ev: ev::MouseEvent| {
                                                 let target: leptos::web_sys::HtmlElement = event_target(&ev);
                                                 let _ = target.style().remove_property("background-color");
                                            }
                                        >
                                            {name}
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    </div>
                    <div style="padding: 10px 16px; background: #f8f9fa; border-top: 1px solid #dadce0; text-align: right; font-size: 0.85rem; color: #666;">
                        "Press 'Esc' to close"
                    </div>
                </div>
            </div>
        </Show>
    }
}
