use leptos::{either::either, prelude::*, tachys::view::ToTemplate};
use leptos_icons::{Icon, IconTemplate, StaticIcon, StaticIconTemplate}; 

#[derive(Clone)]
struct Item {
    pub id: usize,
}

#[derive(Copy, Clone, derive_more::Deref)]
struct DisplayIcon(RwSignal<DisplayIconKind>);

#[derive(Copy, Clone)]
enum DisplayIconKind {
    None,
    Icon,
    Static,
    Template,
    StaticTemplate,
}

#[component]
pub fn App() -> impl IntoView {
    const INIT_NUM_ITEMS: usize = 10;
    let (num_items, set_num_items) = signal(INIT_NUM_ITEMS);
    let (num_cols, set_num_cols) = signal(1);
    let (items, set_items) = signal({
        (0..INIT_NUM_ITEMS)
            .into_iter()
            .map(|id| Item { id })
            .collect::<Vec<_>>()
    });
    let (id, set_id) = signal(INIT_NUM_ITEMS.clone());
    let with_icon = RwSignal::new(DisplayIconKind::None);
    provide_context(DisplayIcon(with_icon));

    Effect::watch(
        num_items,
        move |num_items, _, _| {
            let items_len = items.read_untracked().len();
            let diff = *num_items as isize - items_len as isize;
            if diff > 0 {
                let diff = diff as usize;
                let curr_id = id.get_untracked();
                let new = (0..diff).into_iter().map(|idx| Item { id: idx + curr_id });
                set_items.write().extend(new);
                set_id.update(|id| *id += diff);
            } else if diff < 0 {
                set_items.write().truncate(*num_items);
            }
        },
        false,
    ); 

    view! {
        <div>
            <div>
                <label>
                "rows"
                <input
                    type="number" 
                    min="0"
                    prop:value=num_items
                    on:change:target=move |e| { set_num_items(e.target().value().parse().unwrap()) }
                />
                </label>
                <label>
                "cols"
                <input
                    type="number"
                    min="0"
                    prop:value=num_cols
                    on:change:target=move |e| { set_num_cols(e.target().value().parse().unwrap()) }
                />
                </label>
                <label>
                    "icon"
                    <select
                        on:change:target=move |ev| {
                            let value = match ev.target().value().as_str() {
                                "" => DisplayIconKind::None,
                                "icon" => DisplayIconKind::Icon,
                                "static" => DisplayIconKind::Static,
                                "template" => DisplayIconKind::Template,
                                "static-template" => DisplayIconKind::StaticTemplate,
                                _ => unreachable!("invalid value"),
                            };
                            with_icon.set(value);
                        }
                        prop:value=move || match with_icon.get() {
                            DisplayIconKind::None => "" ,
                            DisplayIconKind::Icon => "icon" ,
                            DisplayIconKind::Static => "static" ,
                            DisplayIconKind::Template => "template" ,
                            DisplayIconKind::StaticTemplate => "static-template" ,
                        }
                    >
                        <option value="">"none"</option>
                        <option value="icon">"<Icon>"</option>
                        <option value="static">"<StaticIcon>"</option>
                        <option value="template">"<IconTemplate>"</option>
                        <option value="static-template">"<StaticIconTemplate>"</option> 
                    </select>
                </label>
            </div>
            <div>
                <DynamicIconTemplate />
            </div>
            <div>
                <div>
                    {move || match with_icon.get() {
                        DisplayIconKind::None => "",
                        DisplayIconKind::Icon => "icon",
                        DisplayIconKind::Static => "static",
                        DisplayIconKind::Template => "template",
                        DisplayIconKind::StaticTemplate => "static-template",
                    }}
                </div>
                <table>
                    <For each=items key=move |item| item.id let:item>
                        <Item item num_cols />
                    </For>
                </table>
            </div>
        </div>
    }
}

#[component]
fn Item(item: Item, num_cols: ReadSignal<usize>) -> impl IntoView {
    view! {
        <tr>
            <For each=move || (0..num_cols.get()) key=|col| col.clone() let:col>
                <td>
                    {item.id}", "{col}
                    <MyIcon />
                </td>
            </For>  
        </tr>
    }
} 


#[component]
fn MyIcon() -> impl IntoView { 
    let display = expect_context::<DisplayIcon>();
    move || either!(
        display.get(),
        DisplayIconKind::None => (),
        DisplayIconKind::Icon => view!{ <Icon icon=icondata::AiAlignCenterOutlined /> },
        DisplayIconKind::Static => view!{ <StaticIcon icon=icondata::AiAlignCenterOutlined /> },
        DisplayIconKind::Template => template! { <IconTemplate icon=icondata::AiAlignCenterOutlined /> },
        DisplayIconKind::StaticTemplate => template! { <StaticIconTemplate icon=icondata::AiAlignCenterOutlined /> }
    )
}

#[component]
fn DynamicIconTemplate() -> impl IntoView {
    let (icon, set_icon) = signal(icondata::AiAlignCenterOutlined);

    template! {
        <div
            on:click=move |e| set_icon(icondata::AiAccountBookFilled)
        >
            "Click me"
            <IconTemplate icon=icon />
        </div>
    }
}