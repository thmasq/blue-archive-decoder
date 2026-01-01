use crate::core::TableData;
use crate::core::text_measurer::measure_text_height;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_resize_observer;
use regex::Regex;
use sqlite_wasm_reader::Value;
use std::hash::Hash;
use std::sync::Arc;

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

#[component]
pub fn TableView(data: Arc<TableData>) -> impl IntoView {
    let (filter_query, set_filter_query) = signal(String::new());

    let data_for_filter = data.clone();
    let data_for_scroller = data.clone();
    let data_for_height = data.clone();
    let data_for_resize = data.clone();

    let default_row_height = 30;

    let (col_width, set_col_width) = signal(100.0);
    let (computed_font_size, set_computed_font_size) = signal(16.0f32);
    let header_ref = NodeRef::<Div>::new();

    let (resize_trigger, set_resize_trigger) = signal(0);

    use_resize_observer(header_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let rect = entry.content_rect();
            let total_width = rect.width();
            let cols = data_for_resize.columns.len().max(1);
            let width_per_col = (total_width - 50.0) / cols as f64;
            let new_width = width_per_col.max(20.0);

            if (new_width - col_width.get_untracked()).abs() > 0.5 {
                set_col_width.set(new_width);
                set_resize_trigger.update(|n| *n += 1);
            }
        }
    });

    Effect::new(move |_| {
        if let Some(el) = header_ref.get() {
            let window = leptos::web_sys::window().unwrap();
            if let Ok(Some(style)) = window.get_computed_style(&el) {
                let font_size_str = style
                    .get_property_value("font-size")
                    .unwrap_or("16px".into());
                let size = font_size_str
                    .trim_end_matches("px")
                    .parse::<f32>()
                    .unwrap_or(16.0);

                if (size - computed_font_size.get_untracked()).abs() > 0.1 {
                    set_computed_font_size.set(size);
                    set_resize_trigger.update(|n| *n += 1);
                }
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
        if *row_idx >= data_for_height.rows.len() {
            return default_row_height;
        }
        let row = &data_for_height.rows[*row_idx];
        let current_col_width = col_width.get() as f32;
        let current_font_size = computed_font_size.get();

        let usable_width = (current_col_width - 10.0).max(10.0);

        let mut max_height = default_row_height as f32;

        for val in row {
            if let Value::Text(text) = val {
                let height = measure_text_height(text, usable_width, current_font_size);
                if height > max_height {
                    max_height = height;
                }
            }
        }
        max_height as usize
    };

    let grid_template = format!("50px {}", "minmax(100px, 1fr) ".repeat(data.columns.len()));

    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">
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
                    default_item_height=default_row_height
                    reset_trigger=resize_trigger
                    children=move |(_, row_idx)| {
                        if *row_idx >= data_for_scroller.rows.len() {
                            return view! { <div>"Error"</div> }.into_any();
                        }
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
                        }.into_any()
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

    // Lazy measurement
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
            style="width: 100%; height: 100%; overflow-y: scroll; will-change: transform;"
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
                                        let top = tree.with(|t| if i == 0 { 0 } else { t.prefix_sum(i - 1) });
                                        let height = tree.with(|t| t.get(i));

                                        view! {
                                            <div
                                                style=format!(
                                                    "position: absolute; width: 100%; top: {}px; height: {}px; {}",
                                                    top,
                                                    height,
                                                    inner_el_style,
                                                )
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
