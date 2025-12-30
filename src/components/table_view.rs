use crate::core::TableData;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_resize_observer;
use regex::Regex;
use sqlite_wasm_reader::Value;
use std::hash::Hash;
use std::sync::Arc;
use wasm_bindgen::JsCast;

#[derive(Clone)]
struct SendWrapper<T>(T);
unsafe impl<T> Send for SendWrapper<T> {}
unsafe impl<T> Sync for SendWrapper<T> {}

impl<T> std::ops::Deref for SendWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[component]
pub fn TableView(data: Arc<TableData>) -> impl IntoView {
    let (filter_query, set_filter_query) = signal(String::new());

    let data_for_filter = data.clone();
    let data_for_scroller = data.clone();
    let data_for_height = data.clone();
    let data_for_resize = data.clone();

    let canvas_ctx = {
        let document = leptos::web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        SendWrapper(ctx)
    };

    let (col_width, set_col_width) = signal(100.0);
    let (computed_font, set_computed_font) = signal("16px sans-serif".to_string());
    let header_ref = NodeRef::<Div>::new();

    use_resize_observer(header_ref, move |entries, _| {
        let rect = entries[0].content_rect();
        let total_width = rect.width();
        let cols = data_for_resize.columns.len().max(1);
        let width_per_col = (total_width - 50.0) / cols as f64;
        set_col_width.set(width_per_col.max(20.0));
    });

    Effect::new(move |_| {
        if let Some(el) = header_ref.get() {
            let window = leptos::web_sys::window().unwrap();
            if let Ok(Some(style)) = window.get_computed_style(&el) {
                let font_size = style
                    .get_property_value("font-size")
                    .unwrap_or("16px".into());
                let font_family = style
                    .get_property_value("font-family")
                    .unwrap_or("sans-serif".into());
                set_computed_font.set(format!("{} {}", font_size, font_family));
            }
        }
    });

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

    let item_height_calc = move |_idx: usize, row_idx: &usize| -> usize {
        let row = &data_for_height.rows[*row_idx];
        let current_col_width = col_width.get();
        let current_font = computed_font.get();

        let usable_width = current_col_width - 14.0;
        let mut max_height = 30;

        canvas_ctx.set_font(&current_font);

        for val in row {
            if let Value::Text(text) = val {
                let height = measure_text_height(&canvas_ctx, text, usable_width);
                if height > max_height {
                    max_height = height;
                }
            }
        }
        max_height
    };

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

            <div
                node_ref=header_ref
                style=format!("display: grid; grid-template-columns: {}; background: #eee; font-weight: bold; border-bottom: 1px solid #999; padding-right: 12px;", grid_template)
            >
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
                    item_height=item_height_calc
                    header=()
                    children=move |(_, row_idx)| {
                        let row: &Vec<Value> = &data_for_scroller.rows[*row_idx];

                        view! {
                            <div style=format!("display: grid; grid-template-columns: {}; border-bottom: 1px solid #eee; height: 100%; align-items: start;", grid_template)>
                                <div style="padding: 6px 4px; color: #888; border-right: 1px solid #eee; font-size: 0.8em; height: 100%; display: flex; align-items: center;">
                                    {(*row_idx + 1).to_string()}
                                </div>
                                {row.iter().map(|val| {
                                    let (txt, color) = match val {
                                        Value::Null => ("NULL".to_string(), "#ccc"),
                                        Value::Integer(i) => (i.to_string(), "#00f"),
                                        Value::Real(f) => (f.to_string(), "#090"),
                                        Value::Text(s) => (s.clone(), "#000"),
                                        Value::Blob(b) => (format!("<Blob {}b>", b.len()), "#a0a"),
                                    };

                                    view! {
                                        <div style=format!("padding: 6px 4px; border-right: 1px solid #eee; overflow: hidden; white-space: pre-wrap; word-wrap: break-word; line-height: 1.25; color: {};", color)
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

fn measure_text_height(
    ctx: &web_sys::CanvasRenderingContext2d,
    text: &str,
    max_width: f64,
) -> usize {
    if text.is_empty() {
        return 30;
    }

    let space_width = ctx.measure_text(" ").unwrap().width();

    let paragraphs = text.split('\n');
    let mut total_lines = 0;

    for paragraph in paragraphs {
        let words = paragraph.split_whitespace();
        let mut current_line_width = 0.0;
        let mut lines_in_paragraph = 1;

        for word in words {
            let word_width = ctx.measure_text(word).unwrap().width();

            if current_line_width + word_width <= max_width {
                current_line_width += word_width + space_width;
            } else {
                lines_in_paragraph += 1;
                if word_width > max_width {
                    lines_in_paragraph += (word_width / max_width).floor() as usize;
                    current_line_width = word_width % max_width;
                } else {
                    current_line_width = word_width + space_width;
                }
            }
        }
        total_lines += lines_in_paragraph;
    }

    (total_lines * 20) + 14
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

#[component]
pub fn VirtualScroller<T, S, K, KN, C, N, H, IH>(
    #[prop()] each: S,
    #[prop()] key: KN,
    #[prop()] children: C,
    #[prop(optional)] header: Option<H>,
    #[prop(optional)] header_height: usize,
    #[prop()] item_height: IH,
    #[prop(default = "")] inner_el_style: &'static str,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static + Clone + Send + Sync,
    KN: (Fn((usize, &T)) -> K) + 'static + Clone + Send + Sync,
    K: Eq + Hash + 'static,
    N: IntoView + 'static,
    S: With<Value = Vec<T>> + Copy + 'static + Send + Sync,
    H: IntoView,
    IH: Fn(usize, &T) -> usize + 'static + Clone + Send + Sync,
{
    let items_len_sig = RwSignal::new(0usize);

    let offsets = Memo::new(move |_| {
        each.with(|items| {
            items_len_sig.set(items.len());

            let mut mapping = Vec::with_capacity(items.len() + 1);
            let mut acc = header_height;
            mapping.push(acc);

            for (i, item) in items.iter().enumerate() {
                acc += item_height(i, item);
                mapping.push(acc);
            }
            mapping
        })
    });

    let inner_height =
        Memo::new(move |_| offsets.with(|offs| *offs.last().unwrap_or(&header_height)));

    let window_height = RwSignal::new(0);
    let scroll_top = RwSignal::new(0);

    let anchor_index = RwSignal::new(0usize);
    let prev_items_count = RwSignal::new(0usize);

    let container = if let Some(node_ref) = node_ref {
        node_ref
    } else {
        NodeRef::new()
    };

    Effect::new(move |_| {
        let offs = offsets.get();
        let count = offs.len();
        let old_count = prev_items_count.get_untracked();

        if count == old_count && count > 1 {
            let idx = anchor_index.get_untracked().min(count - 2);
            if let Some(new_px) = offs.get(idx).copied() {
                if let Some(el) = container.get_untracked() {
                    el.set_scroll_top(new_px as i32);
                    scroll_top.set(new_px);
                }
            }
        }

        prev_items_count.set(count);
    });

    let index_bounds = Memo::new(move |_| {
        let scroll_top = scroll_top.get();
        let window_height = window_height.get();
        let scroll_bottom = scroll_top + window_height;

        offsets.with(|offs| {
            if offs.len() <= 1 {
                return (0, 0);
            }

            let start_index = match offs.binary_search(&scroll_top) {
                Ok(i) => i,
                Err(i) => i.saturating_sub(1),
            };

            let end_index = match offs.binary_search(&scroll_bottom) {
                Ok(i) => i,
                Err(i) => i,
            };

            (start_index, end_index.min(offs.len() - 1))
        })
    });

    let buffer_bounds = Memo::new(move |_| {
        let items_len = items_len_sig.get();
        let (start_index, end_index) = index_bounds.get();
        let buffer_start = if start_index >= 2 { start_index - 2 } else { 0 };
        let buffer_end = (end_index + 2).min(items_len);
        (buffer_start, buffer_end)
    });

    use_resize_observer(container, move |a, _b| {
        let rect = a[0].content_rect();
        window_height.set(rect.height() as usize)
    });

    let force_refresh = RwSignal::new(false);
    Effect::new(move || {
        each.with(|_| {});
        force_refresh.update(|v| {
            *v = !*v;
        });
    });

    view! {
        <div
            node_ref=container
            style="width: 100%; height: 100%; overflow-y: scroll;"
            on:scroll=move |ev| {
                let target: leptos::web_sys::HtmlElement = event_target(&ev);
                let st = target.scroll_top() as usize;

                scroll_top.set(st);

                if let Some(offs) = offsets.try_get_untracked() {
                    if !offs.is_empty() {
                         let idx = match offs.binary_search(&st) {
                            Ok(i) => i,
                            Err(i) => i.saturating_sub(1),
                        };
                        anchor_index.set(idx);
                    }
                }
            }
        >
            <div
                id="scroller"
                style="position: relative;"
                style:height=move || format!("{}px", inner_height.get())
            >
                {header}
                {move || {
                    let children = children.clone();
                    let key = key.clone();
                    force_refresh.get();

                    view! {
                        <For
                            each=move || buffer_bounds.get().0..buffer_bounds.get().1
                            key=move |i| {
                                each.with(|item| key((*i, item.get(*i).unwrap())))
                            }
                            children=move |i| {
                                each.with(|item| {
                                    let item_ref = item.get(i).unwrap();
                                    let (buffer_start, buffer_end) = buffer_bounds.get();

                                    if i >= buffer_start && i <= buffer_end {
                                        view! {
                                            <div
                                                style=move || {
                                                    let top = offsets.with(|offs| offs.get(i).copied().unwrap_or(0));
                                                    let bottom = offsets.with(|offs| offs.get(i+1).copied().unwrap_or(top + 30));
                                                    format!(
                                                        "position: absolute; width: 100%; top: {}px; height: {}px; {}",
                                                        top,
                                                        bottom - top,
                                                        inner_el_style,
                                                    )
                                                }
                                            >
                                                {children((i, item_ref))}
                                            </div>
                                        }.into_any()
                                    } else {
                                        ().into_any()
                                    }
                                })
                            }
                        />
                    }
                }}
            </div>
        </div>
    }
}
