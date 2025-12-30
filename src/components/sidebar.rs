use crate::core::TableData;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[component]
pub fn Sidebar(
    tables: Signal<HashMap<String, Arc<TableData>>>,
    selected: Signal<Option<String>>,
    on_select: WriteSignal<Option<String>>,
) -> impl IntoView {
    let sorted_names = move || {
        let mut names: Vec<String> = tables.get().keys().cloned().collect();
        names.sort();
        names
    };

    view! {
        <div style="flex: 1; overflow-y: auto;">
            <ul style="list-style: none; padding: 0; margin: 0;">
                <For
                    each=sorted_names
                    key=|name| name.clone()
                    children=move |name| {
                        let is_selected = selected.get() == Some(name.clone());
                        let bg = if is_selected { "#ddd" } else { "transparent" };
                        let n_clone = name.clone();

                        view! {
                            <li
                                on:click=move |_| on_select.set(Some(n_clone.clone()))
                                style=format!("padding: 8px 10px; cursor: pointer; background: {}; border-bottom: 1px solid #eee;", bg)
                            >
                                {name}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
