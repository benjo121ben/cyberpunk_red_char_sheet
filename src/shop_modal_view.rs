use std::vec;
use leptos::prelude::*;

use crate::gear::{ShopItem, GearData};

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct ShopModalData {
    visible: bool, 
    pub title: String, 
    pub description: String
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ShopItemVisualData {
    name: String,
    description: String,
    price: i32,
    type_data: String
}

impl ShopItemVisualData {
    pub fn from(item: &impl ShopItem) -> Self {
        Self { 
            name: item.get_name().clone(),
            description: item.get_description().clone(),
            price: item.get_price(),
            type_data: item.get_type().clone(),
        }
    }
}

impl ShopModalData {
    pub fn show(self: &mut Self) {
        self.visible = true;
    }

    pub fn hide(self: &mut Self) {
        self.visible = false;
        self.title = String::new();
        self.description = String::new();
    }
}

#[component]
pub fn ShopModalView(data: RwSignal<ShopModalData>) -> AnyView {
    view! {
        <Show when=move||data.get().visible>
            <ShopContent data=data.clone()/>
        </Show>
    }.into_any()
}


#[component]
pub fn ShopContent(data: RwSignal<ShopModalData>) -> AnyView {
    let current_tab: RwSignal<usize> = RwSignal::new(0);
    let currently_selected_index: RwSignal<usize> = RwSignal::new(0);
    let tabs = vec!["Weapons", "Ammo", "Armor", "Cyberware", "Drugs", "Gear", "Hardware", "Programs"];

    view! {
        <div class="modal" on:click=move |_| data.update(|data| data.hide())>
            <div class="modal_content" on:click=move |ev| { ev.stop_propagation();}>
                <div class="tabs_list">
                    {tabs.into_iter().enumerate().map(|(i, tab_name)| {
                        view! {
                            <div class="tab" 
                                on:click=move|_| {
                                    currently_selected_index.set(0);
                                    current_tab.set(i);
                                }
                                class:selected_tab=move|| current_tab.get() == i> 
                                    {tab_name}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <hr/>
                <ShopList current_tab currently_selected_index/>
                //<p inner_html={move|| data.get().description}/>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn ShopList(current_tab: RwSignal<usize>, currently_selected_index: RwSignal<usize>) -> AnyView {
    let gear_data: GearData = use_context().expect("Expecting gear data existence");
    let current_items_memo = Memo::new(move |_| {
        let mut list = match current_tab.get() {
            0 => gear_data.weapons.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            1 => gear_data.ammunition.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            2 => gear_data.armor.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            3 => gear_data.cyberware.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            4 => gear_data.drugs.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            5 => gear_data.items.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            6 => gear_data.cyberdeck_hardware.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            7 => gear_data.programs.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            _ => panic!("this shop tab does not have any data")
        };
        list.sort_by(|val,val2| val.name.cmp(&val2.name));
        list
    });

    let currenty_selected_item = Memo::new(move |_| {
        current_items_memo.read().get(currently_selected_index.get()).expect("item to exist").clone()
    });

    view! {
        <div class="shop_content">
            <div class="name_list_wrapper">
                <div class="name_list">
                    <For each=move||{current_items_memo.get().into_iter().enumerate().collect::<Vec<_>>()}
                        key=move|(_, shop_item)|shop_item.name.clone()
                        children=move|(index, shop_item)| {
                            let name = shop_item.name.clone();
                            view!{
                                <span
                                    class:span_selected=move||{currently_selected_index.get()==index}
                                    on:click=move|_|{currently_selected_index.set(index)}
                                >{move || name.clone()}</span>
                            }
                        }
                    />
                </div>
            </div>
            <div class="selected_store_item">{move || {
                let name = currenty_selected_item.read().name.clone();
                let description = currenty_selected_item.read().description.clone();
                let price = currenty_selected_item.read().price.clone();
                view!{
                    <div class="shop_item">
                        <div class="shop_item_header_part">
                            <span class="shop_item_name">{move || name.clone()}</span> 
                            <span class="shop_item_price">{move || price.clone()}eb</span>
                        </div>
                        <div class="shop_item_description" inner_html=move || description.clone()/>
                        
                    </div>
                }
            }}</div>
        </div>
    }.into_any()
}