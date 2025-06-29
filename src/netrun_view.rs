use cp_char_data::gear::{get_map_key, GearData};
use leptos::prelude::*;

use crate::{help::{get_char_signal_from_ctx, reduce_or_remove_items_in_map}, info_modal_view::SimpleModalData};

#[component]
pub fn NetrunView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let gear_data: GearData = use_context().expect("expecting gear data to exist. | NetrunView");
    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("allitemsview: simple modal should exist");
    let interface_modal = Memo::new(move|_| {char_signal.read().role_abilities.netrunner});
    view! {
        <div class="netrun_view">
            <span>{move || format!("Interface {} - Actions: {}", interface_modal(), (interface_modal() as f32 / 3.0).ceil() + 1.0)}</span>
            <For each=move|| {char_signal.read().programs.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = StoredValue::new(gear_data.programs.iter().find(|find_item| {
                        let map_key = get_map_key(*find_item);
                        map_key == key
                    }).cloned().expect("expecting item to exist"));
                    let key_clone = key.clone(); 
                    let item_amount_memo = Memo::new(move |_| {char_signal.read().programs.get(&key_clone).expect("expecting item amount to exist").to_string()});
                    view! {
                        <div class="gear_view program_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item.get_value().name.clone();
                                data.description = find_item.get_value().description.clone();
                                data.show();
                            })
                        >
                            <span>NAME</span>
                            <span>ATK</span>
                            <span>DEF</span>
                            <span>REZ</span>
                            <span>
                                {move || item_amount_memo()}x
                                {move || find_item.get_value().name.to_string()}
                            </span>
                            <span>
                                {move || find_item.get_value().atk}
                            </span>
                            <span>
                                {move || find_item.get_value().def}
                            </span>
                            <span>
                                {move || find_item.get_value().rez}
                            </span>
                            <button on:click=move|ev|{
                                ev.stop_propagation();
                                char_signal.update(|c| {
                                    reduce_or_remove_items_in_map(&mut c.programs, &key)
                                })
                            }>
                                X
                            </button>
                        </div>
                    }
                }
            />
            <hr style="width:90%"/>
            <For each=move|| {char_signal.read().cyberdeck_hardware.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = StoredValue::new(gear_data.cyberdeck_hardware.iter().find(|find_item| {
                        let map_key = get_map_key(*find_item);
                        map_key == key
                    }).cloned().expect("expecting item to exist"));
                    let key_clone = key.clone(); 
                    let item_amount_memo = Memo::new(move |_| {char_signal.read().cyberdeck_hardware.get(&key_clone).expect("expecting item amount to exist").to_string()});
                    view! {
                        <div class="gear_view program_view"
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
                                    reduce_or_remove_items_in_map(&mut c.cyberdeck_hardware, &key)
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