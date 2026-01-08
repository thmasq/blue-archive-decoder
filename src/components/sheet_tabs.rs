use crate::core::TableData;
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[component]
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn SheetTabs(
    tables: Signal<HashMap<String, Arc<TableData>>>,
    selected: Signal<Option<String>>,
    on_select: WriteSignal<Option<String>>,
    #[prop(into)] on_add: Callback<()>,
) -> impl IntoView {
    let sorted_names = move || {
        let mut names: Vec<String> = tables.get().keys().cloned().collect();
        names.sort();
        names
    };

    view! {
        <div style="height: 36px; background: #f1f3f4; border-top: 1px solid #dadce0; display: flex; align-items: flex-end; overflow-x: auto; padding-left: 40px; box-sizing: border-box; position: relative;">
            <div
                style="position: absolute; left: 0; bottom: 0; height: 35px; width: 40px; display: flex; align-items: center; justify-content: center; background: #f1f3f4; border-right: 1px solid #dadce0; z-index: 10; cursor: pointer; transition: background 0.2s;"
                on:click=move |_| on_add.run(())
                title="Open Table Selector (Press 't')"
                on:mouseover=move |ev: leptos::ev::MouseEvent| {
                     let target: leptos::web_sys::HtmlElement = event_target(&ev);
                     let _ = target.style().set_property("background-color", "#e8eaed");
                }
                on:mouseout=move |ev: leptos::ev::MouseEvent| {
                     let target: leptos::web_sys::HtmlElement = event_target(&ev);
                     let _ = target.style().remove_property("background-color");
                }
            >
                <span style="font-size: 1.4rem; color: #5f6368; line-height: 1;">"+"</span>
            </div>

            <For
                each=sorted_names
                key=|name| name.clone()
                children=move |name| {
                    let is_selected = selected.get() == Some(name.clone());
                    let n_clone = name.clone();

                    let (bg, color, font_weight, box_shadow) = if is_selected {
                        ("#ffffff", "#0f9d58", "bold", "0 1px 3px rgba(0,0,0,0.1)")
                    } else {
                        ("transparent", "#5f6368", "normal", "none")
                    };

                    view! {
                        <div
                            on:click=move |_| on_select.set(Some(n_clone.clone()))
                            style=format!(
                                "padding: 6px 18px; cursor: pointer; background: {}; color: {}; font-weight: {}; font-size: 0.85rem; border-right: 1px solid #dadce0; border-top: 1px solid transparent; min-width: 60px; text-align: center; white-space: nowrap; user-select: none; box-shadow: {}; margin-bottom: 1px; height: 20px; flex-shrink: 0;",
                                bg, color, font_weight, box_shadow
                            )
                        >
                            {name}
                        </div>
                    }
                }
            />
        </div>
    }
}
