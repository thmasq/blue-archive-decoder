use crate::core::text_measurer::measure_text_height;
use crate::core::{FONT_SIZE, TableData};
use leptos::ev;
use leptos::html::Div;
use leptos::prelude::window;
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_resize_observer};
use regex::Regex;
use sqlite_wasm_reader::Value;
use std::hash::Hash;
use std::sync::Arc;
use wasm_bindgen::JsCast;

const MIN_COL_WIDTH: f64 = 50.0;

#[derive(Clone, Debug)]
pub struct FenwickTree {
    tree: Vec<i64>,
    size: usize,
}

impl FenwickTree {
    pub fn new(size: usize, default_value: i64) -> Self {
        let mut tree = vec![0i64; size + 1];
        for i in 1..=size {
            tree[i] += default_value;
            let parent = i + (i & (!i + 1));
            if parent <= size {
                tree[parent] += tree[i];
            }
        }
        Self { tree, size }
    }

    pub fn update(&mut self, index: usize, delta: i64) {
        if index >= self.size {
            return;
        }
        let mut i = index + 1;
        while i <= self.size {
            self.tree[i] += delta;
            i += i & (!i + 1);
        }
    }

    pub fn prefix_sum(&self, index: usize) -> i64 {
        let mut i = (index + 1).min(self.size);
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & (!i + 1);
        }
        sum
    }

    pub fn get(&self, index: usize) -> i64 {
        if index >= self.size {
            return 0;
        }
        let current = self.prefix_sum(index);
        let prev = if index == 0 {
            0
        } else {
            self.prefix_sum(index - 1)
        };
        current - prev
    }

    pub fn total_sum(&self) -> i64 {
        if self.size == 0 {
            0
        } else {
            self.prefix_sum(self.size - 1)
        }
    }

    pub fn lower_bound(&self, value: i64) -> usize {
        if self.size == 0 {
            return 0;
        }
        let mut idx = 0;
        let mut current_sum = 0;
        let mut bit_mask = 1;
        while (bit_mask << 1) <= self.size {
            bit_mask <<= 1;
        }

        while bit_mask > 0 {
            let t_idx = idx + bit_mask;
            if t_idx <= self.size {
                if current_sum + self.tree[t_idx] <= value {
                    idx = t_idx;
                    current_sum += self.tree[t_idx];
                }
            }
            bit_mask >>= 1;
        }
        idx
    }
}

#[derive(Clone, Copy)]
struct ResizeState {
    col_idx: usize,
    start_x: f64,
    start_width: f64,
}

#[component]
pub fn TableView(data: Arc<TableData>) -> impl IntoView {
    let (filter_query, set_filter_query) = signal(String::new());

    let data_for_filter = data.clone();
    let data_for_scroller = data.clone();
    let data_for_height = data.clone();

    let initial_col_count = data.columns.len().max(1);

    let (display_widths, set_display_widths) = signal(vec![100.0; initial_col_count]);
    let (measured_widths, set_measured_widths) = signal(vec![100.0; initial_col_count]);
    let (manual_resize_triggered, set_manual_resize_triggered) = signal(false);

    let default_row_height = 24;

    let header_ref = NodeRef::<Div>::new();
    let scroll_view_ref = NodeRef::<Div>::new();
    let (scrollbar_width, set_scrollbar_width) = signal(12.0);

    let (resizing, set_resizing) = signal::<Option<ResizeState>>(None);
    let (resize_trigger, set_resize_trigger) = signal(0);

    let _ = use_event_listener(window(), ev::mousemove, move |ev| {
        if let Some(state) = resizing.get() {
            let current_x = ev.client_x() as f64;
            let delta = current_x - state.start_x;
            let new_width = (state.start_width + delta).max(MIN_COL_WIDTH);

            set_display_widths.update(|widths| {
                if state.col_idx < widths.len() {
                    widths[state.col_idx] = new_width;
                }
            });
        }
    });

    let _ = use_event_listener(window(), ev::mouseup, move |_| {
        if resizing.get().is_some() {
            set_resizing.set(None);
            set_manual_resize_triggered.set(true);

            let current_display = display_widths.get();
            set_measured_widths.set(current_display);

            set_resize_trigger.update(|n| *n += 1);
        }
    });

    use_resize_observer(header_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            if !manual_resize_triggered.get_untracked() {
                let rect = entry.content_rect();
                let total_width = rect.width();
                let cols = initial_col_count;
                let width_per_col = (total_width / cols as f64).max(80.0).min(200.0);

                set_display_widths.update(|widths| {
                    let current_avg = widths.iter().sum::<f64>() / widths.len() as f64;
                    if (current_avg - width_per_col).abs() > 0.5 {
                        *widths = vec![width_per_col; cols];

                        set_measured_widths.set(widths.clone());
                        set_resize_trigger.update(|n| *n += 1);
                    }
                });
            }
        }
    });

    Effect::new(move |_| {
        let window = leptos::web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let div = document.create_element("div").unwrap();
        div.set_attribute(
            "style",
            "width: 100px; height: 100px; overflow: scroll; position: absolute; top: -9999px;",
        )
        .unwrap();
        body.append_child(&div).unwrap();

        let div_html = div.dyn_ref::<leptos::web_sys::HtmlElement>().unwrap();
        let width = div_html.offset_width() - div.client_width();

        div.remove();
        set_scrollbar_width.set(width as f64);
    });

    let _ = use_event_listener(scroll_view_ref, leptos::ev::scroll, move |ev| {
        if let Some(header) = header_ref.get() {
            let target: leptos::web_sys::HtmlElement = event_target(&ev);
            header.set_scroll_left(target.scroll_left());
        }
    });

    let filtered_rows = Memo::new(move |_| {
        let query = filter_query.get();
        let rows = &data_for_filter.rows;

        if query.is_empty() {
            return (0..rows.len()).collect::<Vec<_>>();
        }

        let re = Regex::new(&format!("(?i){}", query))
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
        if *row_idx >= data_for_height.rows.len() {
            return default_row_height;
        }
        let row = &data_for_height.rows[*row_idx];

        let widths = measured_widths.get();

        let mut max_height = default_row_height as f32;

        for (col_idx, val) in row.iter().enumerate() {
            if let Value::Text(text) = val {
                let col_w = if col_idx < widths.len() {
                    widths[col_idx]
                } else {
                    MIN_COL_WIDTH
                };

                let usable_width = (col_w as f32 - 11.0).max(10.0);

                let height = measure_text_height(text, usable_width, FONT_SIZE);
                if height > max_height {
                    max_height = height;
                }
            }
        }
        max_height as usize
    };

    let grid_template = move || {
        let widths = display_widths.get();
        let cols_str = widths
            .iter()
            .map(|w| format!("{}px", w))
            .collect::<Vec<_>>()
            .join(" ");
        format!("46px {}", cols_str)
    };

    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">

            <div style="background: #ffffff; border-bottom: 1px solid #dadce0; padding: 4px 8px; display: flex; align-items: center; gap: 8px; height: 32px; box-sizing: border-box;">
                <div style="color: #9aa0a6; font-style: italic; font-weight: bold; font-family: serif; user-select: none;">
                    "fx"
                </div>
                <div style="flex: 1; position: relative;">
                     <input
                        type="text"
                        placeholder="Search / Filter..."
                        prop:value=move || filter_query.get()
                        on:input=move |ev| set_filter_query.set(event_target_value(&ev))
                        style=format!("width: 100%; border: none; background: transparent; font-size: {}px; outline: none; padding: 4px;", FONT_SIZE)
                    />
                </div>
                <div style="font-size: 0.8rem; color: #5f6368; user-select: none;">
                    {move || format!("{} records", filtered_rows.get().len())}
                </div>
            </div>

            <div
                node_ref=header_ref
                style=move || format!("display: grid; grid-template-columns: {}; background: #f8f9fa; border-bottom: 1px solid #c0c0c0; padding-right: {}px; overflow-x: hidden; user-select: none;", grid_template(), scrollbar_width.get())
            >
                <div style="border-right: 1px solid #c0c0c0; background: #f8f9fa;"></div>

                {
                    data.columns.iter().enumerate().map(|(i, col)| view! {
                        <div style="position: relative; padding: 4px 6px; border-right: 1px solid #c0c0c0; color: #5f6368; font-weight: 700; font-size: 11px; display: flex; align-items: center; justify-content: center; height: 24px;">
                            <span style="overflow: hidden; text-overflow: ellipsis; white-space: nowrap; width: 100%; text-align: center;">
                                {col.clone()}
                            </span>
                            <div
                                style="position: absolute; right: 0; top: 0; bottom: 0; width: 6px; cursor: col-resize; z-index: 10;"
                                on:mousedown=move |ev| {
                                    let start_x = ev.client_x() as f64;
                                    let current_width = display_widths.with(|w| w.get(i).copied().unwrap_or(MIN_COL_WIDTH));
                                    set_resizing.set(Some(ResizeState {
                                        col_idx: i,
                                        start_x,
                                        start_width: current_width,
                                    }));
                                    ev.prevent_default();
                                }
                                on:mouseover=move |ev: ev::MouseEvent| {
                                    let target: leptos::web_sys::HtmlElement = event_target(&ev);
                                    let _ = target.style().set_property("background-color", "#4285f4");
                                }
                                on:mouseout=move |ev: ev::MouseEvent| {
                                    let target: leptos::web_sys::HtmlElement = event_target(&ev);
                                    let _ = target.style().remove_property("background-color");
                                }
                            ></div>
                        </div>
                    }).collect::<Vec<_>>()
                }
            </div>

            <div style="flex: 1; overflow-y: hidden; background: #fff;">
                <VirtualScroller
                    each=filtered_rows
                    key=move |(_, idx)| *idx
                    item_height=item_height_calc
                    default_item_height=default_row_height
                    reset_trigger=resize_trigger
                    node_ref=scroll_view_ref
                    children=move |(_, row_idx)| {
                        if *row_idx >= data_for_scroller.rows.len() {
                            return view! { <div>"Error"</div> }.into_any();
                        }
                        let row: &Vec<Value> = &data_for_scroller.rows[*row_idx];

                        view! {
                            <div style=move || format!("display: grid; grid-template-columns: {}; height: 100%; align-items: start;", grid_template())>
                                <div style="background: #f8f9fa; border-right: 1px solid #c0c0c0; border-bottom: 1px solid #ccc; color: #5f6368; font-size: 11px; display: flex; align-items: center; justify-content: center; height: 100%; user-select: none;">
                                    {(*row_idx + 1).to_string()}
                                </div>
                                {row.iter().map(|val| {
                                    let (txt, color, align) = match val {
                                        Value::Null => ("".to_string(), "#ccc", "left"),
                                        Value::Integer(i) => (i.to_string(), "#1155cc", "right"),
                                        Value::Real(f) => (f.to_string(), "#090", "right"),
                                        Value::Text(s) => (s.clone(), "#000", "left"),
                                        Value::Blob(b) => (format!("<Blob {}b>", b.len()), "#a0a", "center"),
                                    };

                                    view! {
                                        <div style=format!("box-sizing: border-box; padding: 4px 5px; border-right: 1px solid #e0e0e0; border-bottom: 1px solid #e0e0e0; overflow: hidden; white-space: pre-wrap; word-wrap: break-word; line-height: 1.3; font-size: {}px; color: {}; text-align: {}; height: 100%;", FONT_SIZE, color, align)
                                            title=txt.clone()>
                                            {txt.clone()}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                />
            </div>
        </div>
    }
}

fn row_matches(row: &[Value], re: &Regex) -> bool {
    for val in row {
        if let Value::Text(s) = val {
            if re.is_match(s) {
                return true;
            }
        }
    }
    false
}

#[component]
pub fn VirtualScroller<T, S, K, KN, C, N, IH>(
    #[prop()] each: S,
    #[prop()] key: KN,
    #[prop()] children: C,
    #[prop()] item_height: IH,
    #[prop()] default_item_height: usize,
    #[prop(optional)] reset_trigger: Option<ReadSignal<usize>>,
    #[prop(default = "")] inner_el_style: &'static str,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
) -> impl IntoView
where
    C: Fn((usize, &T)) -> N + 'static + Clone + Send + Sync,
    KN: Fn((usize, &T)) -> K + 'static + Clone + Send + Sync,
    K: Eq + Hash + 'static,
    N: IntoView + 'static,
    S: With<Value = Vec<T>> + Copy + 'static + Send + Sync,
    IH: Fn(usize, &T) -> usize + 'static + Clone + Send + Sync,
{
    let tree = RwSignal::new(FenwickTree::new(0, default_item_height as i64));
    let measured_flags = RwSignal::new(Vec::<bool>::new());

    let window_height = RwSignal::new(0usize);
    let scroll_top = RwSignal::new(0usize);
    let container = node_ref.unwrap_or_else(NodeRef::new);

    Effect::new(move |_| {
        if let Some(trigger) = reset_trigger {
            trigger.get();
        }

        each.with(|items| {
            let len = items.len();
            batch(move || {
                tree.set(FenwickTree::new(len, default_item_height as i64));
                measured_flags.set(vec![false; len]);

                scroll_top.set(0);

                if let Some(el) = container.get() {
                    el.set_scroll_top(0);
                }
            });
        });
    });

    let inner_height = Memo::new(move |_| tree.with(|t| t.total_sum()));

    use_resize_observer(container, move |entries, _| {
        if let Some(entry) = entries.first() {
            window_height.set(entry.content_rect().height() as usize);
        }
    });

    let index_bounds = Memo::new(move |_| {
        let st = scroll_top.get() as i64;
        let wh = window_height.get() as i64;
        let sb = st + wh;

        tree.with(|t| {
            if t.size == 0 {
                return (0, 0);
            }

            let start = t.lower_bound(st);
            let end = t.lower_bound(sb);

            (
                start.min(t.size.saturating_sub(1)),
                end.min(t.size.saturating_sub(1)),
            )
        })
    });

    let buffer_bounds = Memo::new(move |_| {
        let items_len = each.with(|i| i.len());
        let (start, end) = index_bounds.get();

        let buffer_start = start.saturating_sub(2);
        let buffer_end = (end + 2).min(items_len);

        (buffer_start, buffer_end)
    });

    Effect::new(move |_| {
        let (start, end) = buffer_bounds.get();
        let mut updates = Vec::new();

        each.with(|items| {
            measured_flags.with_untracked(|flags| {
                for i in start..end {
                    if i < items.len() && i < flags.len() && !flags[i] {
                        let real_height = item_height(i, &items[i]) as i64;
                        updates.push((i, real_height));
                    }
                }
            });
        });

        if !updates.is_empty() {
            batch(move || {
                tree.update(|t| {
                    for (idx, new_h) in &updates {
                        let diff = new_h - t.get(*idx);
                        if diff != 0 {
                            t.update(*idx, diff);
                        }
                    }
                });

                measured_flags.update(|flags| {
                    for (idx, _) in &updates {
                        if *idx < flags.len() {
                            flags[*idx] = true;
                        }
                    }
                });
            });
        }
    });

    view! {
        <div
            node_ref=container
            style="width: 100%; height: 100%; overflow-y: scroll; overflow-x: auto; will-change: transform;"
            on:scroll=move |ev| {
                let target: leptos::web_sys::HtmlElement = event_target(&ev);
                scroll_top.set(target.scroll_top() as usize);
            }
        >
            <div
                style="position: relative;"
                style:height=move || format!("{}px", inner_height.get())
            >
                {move || {
                    let children = children.clone();
                    let key_fn = key.clone();
                    let (start, end) = buffer_bounds.get();

                    view! {
                        <For
                            each=move || start..end
                            key=move |i| {
                                each.with(|items| {
                                    items
                                        .get(*i)
                                        .map(|item| key_fn((*i, item)))
                                })
                            }
                            children=move |i| {
                                each.with(|items| {
                                    if let Some(item) = items.get(i) {
                                        view! {
                                            <div
                                                style=move || {
                                                    let top = tree.with(|t| if i == 0 { 0 } else { t.prefix_sum(i - 1) });
                                                    let height = tree.with(|t| t.get(i));
                                                    format!(
                                                        "position: absolute; width: 100%; top: {}px; height: {}px; {}",
                                                        top,
                                                        height,
                                                        inner_el_style,
                                                    )
                                                }
                                            >
                                                {children((i, item))}
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
