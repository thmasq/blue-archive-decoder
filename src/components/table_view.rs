use crate::core::TableData;
use leptos::prelude::*;
use leptos_virtual_scroller::VirtualScroller;
use regex::Regex;
use sqlite_wasm_reader::Value;
use std::sync::Arc;

#[component]
pub fn TableView(data: Arc<TableData>) -> impl IntoView {
    let (filter_query, set_filter_query) = signal(String::new());

    let data_for_filter = data.clone();
    let data_for_scroller = data.clone();

    let filtered_rows = Memo::new(move |_| {
        let query = filter_query.get();
        let rows = &data_for_filter.rows;

        if query.is_empty() {
            return (0..rows.len()).collect::<Vec<_>>();
        }

        let re = Regex::new(&query)
            .ok()
            .or_else(|| Regex::new(&regex::escape(&query)).ok());

        if let Some(re) = re {
            rows.iter()
                .enumerate()
                .filter(|(_, row)| row_matches(row, &re))
                .map(|(i, _)| i)
                .collect()
        } else {
            vec![]
        }
    });

    let grid_template = format!("50px {}", "minmax(100px, 1fr) ".repeat(data.columns.len()));

    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">
            // Toolbar
            <div style="padding: 10px; border-bottom: 1px solid #ccc; background: #fff;">
                <strong>{data.name.clone()}</strong>
                <span style="margin: 0 10px; color: #666;">
                    {move || format!("{} rows", filtered_rows.get().len())}
                </span>
                <input
                    type="text"
                    placeholder="Search (Regex supported)..."
                    prop:value=move || filter_query.get()
                    on:input=move |ev| set_filter_query.set(event_target_value(&ev))
                    style="padding: 4px; width: 200px;"
                />
            </div>

            <div style=format!("display: grid; grid-template-columns: {}; background: #eee; font-weight: bold; border-bottom: 1px solid #999; padding-right: 12px;", grid_template)>
                <div style="padding: 4px; border-right: 1px solid #ccc;">"#"</div>
                {data.columns.iter().map(|col| view! {
                    <div style="padding: 4px; border-right: 1px solid #ccc; overflow: hidden; text-overflow: ellipsis;">
                        {col.clone()}
                    </div>
                }).collect::<Vec<_>>()}
            </div>

            <div style="flex: 1; overflow-y: auto;">
                <VirtualScroller
                    each=filtered_rows
                    key=move |(_, idx)| *idx
                    item_height=30
                    header=()
                    children=move |(_, row_idx)| {
                        // Use the explicit clone here
                        let row: &Vec<Value> = &data_for_scroller.rows[*row_idx];

                        view! {
                            <div style=format!("display: grid; grid-template-columns: {}; border-bottom: 1px solid #eee; height: 30px; align-items: center;", grid_template)>
                                <div style="padding: 0 4px; color: #888; border-right: 1px solid #eee; font-size: 0.8em;">
                                    {(*row_idx + 1).to_string()}
                                </div>
                                {row.iter().map(|val| {
                                    // Generate owned Strings.
                                    // This is required because the view needs to own the data to display it,
                                    // and we cannot return references to local variables from the match block.
                                    let (txt, color) = match val {
                                        Value::Null => ("NULL".to_string(), "#ccc"),
                                        Value::Integer(i) => (i.to_string(), "#00f"),
                                        Value::Real(f) => (f.to_string(), "#090"),
                                        Value::Text(s) => (s.clone(), "#000"),
                                        Value::Blob(b) => (format!("<Blob {}b>", b.len()), "#a0a"),
                                    };

                                    view! {
                                        <div style=format!("padding: 0 4px; border-right: 1px solid #eee; overflow: hidden; white-space: nowrap; text-overflow: ellipsis; color: {};", color)
                                            title=txt.clone()>
                                            {txt.clone()}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

fn row_matches(row: &[Value], re: &Regex) -> bool {
    for val in row {
        let text = match val {
            Value::Text(s) => s.as_str(),
            Value::Integer(i) => return re.is_match(&i.to_string()),
            Value::Real(f) => return re.is_match(&f.to_string()),
            _ => "",
        };
        if re.is_match(text) {
            return true;
        }
    }
    false
}
