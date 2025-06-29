use cp_char_data::gear::GearData;
use leptos::prelude::*;

use crate::{help::{get_char_signal_from_ctx, reduce_or_remove_items_in_map}, info_modal_view::SimpleModalData};

#[component]
pub fn NetrunView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let gear_data: GearData = use_context().expect("expecting gear data to exist. | NetrunView");
    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("allitemsview: simple modal should exist");
    view! {
        <div class="netrun_view">
            <For each=move|| {char_signal.read().programs.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = StoredValue::new(gear_data.programs.iter().find(|find_item| {
                        let changed_name = find_item.name.to_lowercase().replace(" ", "_");
                        changed_name == key
                    }).cloned().expect("expecting item to exist"));
                    let key_clone = key.clone(); 
                    let item_amount_memo = Memo::new(move |_| {char_signal.read().programs.get(&key_clone).expect("expecting item amount to exist").to_string()});
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item.get_value().name.clone();
                                data.description = find_item.get_value().description.clone();
                                data.show();
                            })
                        >
                            <span>
                                {move || item_amount_memo()}x
                                {move || find_item.get_value().name.to_string()}
                            </span>
                            <button on:click=move|ev|{
                                ev.stop_propagation();
                                char_signal.update(|c| {
                                    reduce_or_remove_items_in_map(&mut c.gear, &key)
                                })
                            }>
                                X
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}