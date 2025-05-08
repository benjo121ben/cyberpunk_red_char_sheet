use std::vec;
use leptos::prelude::*;
use cp_char_data::{char::GearType, gear::{GearData, ShopItemVisualData}};
use crate::help::get_char_signal_from_ctx;

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct ShopModalData {
    visible: bool, 
    pub title: String, 
    pub description: String
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
    let cyberpunk_signal = get_char_signal_from_ctx();
    let gear_data: GearData = use_context().expect("Expecting gear data existence");
    
    let current_tab: RwSignal<(usize, String)> = RwSignal::new((0, "Weapons".to_string()));
    let currently_selected_index: RwSignal<usize> = RwSignal::new(0);
    let head_armor = RwSignal::new(false);
    let variant_options = Memo::new(move |_| {
        
        if current_tab.get().1.as_str() == "Ammo" {
            vec!["rifle", "medium_pistol", "heavy_pistol", "v_heavy_pistol", "slug", "shell", "grenade", "rocket", "arrow", "paintball" ]
                .into_iter()
                .map(|val|val.to_string())
                .collect::<Vec<String>>()
        }
        else {
            Vec::new()
        }
    });
    let current_variant:RwSignal<String> = RwSignal::new("rifle".to_string());
    
    let tabs = vec!["Weapons", "Ammo", "Armor", "Cyberware", "Drugs", "Gear", "Fashion", "Hardware", "Programs"];

    let current_items_memo: Memo<Vec<ShopItemVisualData>> = Memo::new(move |_| {
        let gear_data: GearData = use_context().expect("Expecting gear data existence");
        let mut list = match current_tab.get().1.as_str() {
            "Weapons" => gear_data.weapons.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Ammo" => gear_data.ammunition.iter().filter(|ammo| ammo.caliber == current_variant.get()).map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Armor" => gear_data.armor.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Cyberware" => gear_data.cyberware.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Drugs" => gear_data.drugs.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Gear" => gear_data.items.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Fashion" => gear_data.fashion.clone(),
            "Hardware" => gear_data.cyberdeck_hardware.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            "Programs" => gear_data.programs.iter().map(|val| ShopItemVisualData::from(val)).collect::<Vec<_>>(),
            _ => panic!("this shop tab does not have any data")
        };
        list.sort_by(|val,val2| val.name.cmp(&val2.name));
        list
    });

    let currenty_selected_item = Memo::new(move |_| {
        current_items_memo.read().get(currently_selected_index.get()).expect("item to exist").clone()
    });

    let check_money_and_reduce = move |price: i32| -> bool{
        let ret = cyberpunk_signal.read().money >= price;
        if ret {
            cyberpunk_signal.write().money -= price;
        }
        ret
    };

    let give_item = move |must_pay: bool| {
        let current_item = currenty_selected_item.get();
        match current_tab.get().1.as_str() {
            "Weapons" => {
                let bought_item = gear_data.weapons.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.weapons.push(bought_item));
            },
            
            "Ammo" => {
                let bought_item = gear_data.ammunition.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|{
                    let amount = bought_item.only_one.then(||1).or(Some(10)).unwrap();
                    let changed_name = bought_item.name.to_lowercase().replace(" ", "_");
                    if c.ammo.get_mut(&changed_name).and_then(|val| Some(*val += amount)).is_none() {
                        c.ammo.insert(changed_name, amount);
                    }
                });
            },
            
            "Armor" => {
                let mut bought_item = gear_data.armor.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");

                bought_item.armor_data.sp_current = bought_item.armor_data.sp;
                bought_item.armor_data.head = head_armor.get();
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.armors.push(bought_item));
            },
            
            "Cyberware" =>{
                let bought_item = gear_data.cyberware.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return; 
                }
                cyberpunk_signal.update(|c|c.cyberware.push(bought_item));
            },
            
            "Drugs" => {
                let bought_item = gear_data.drugs.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.add_gear(GearType::Drugs, bought_item.name));
            },

            "Gear" => {
                let bought_item = gear_data.items.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.add_gear(GearType::Gear, bought_item.name));
            },
            
            "Hardware" => {
                let bought_item = gear_data.cyberdeck_hardware.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.add_gear(GearType::Hardware, bought_item.name));

            },
            
            "Programs" => {
                let bought_item = gear_data.programs.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.price) {
                    return;
                }
                cyberpunk_signal.update(|c|c.add_gear(GearType::Programs, bought_item.name));
            },
            
            _ => panic!("this shop tab does not have any data")
        };
    };


    view! {
        <div class="modal" on:click=move |_| data.update(|data| data.hide())>
            <div class="modal_content" on:click=move |ev| { ev.stop_propagation();}>
                <div class="tabs_list">
                    {tabs.into_iter().enumerate().map(|(i, tab_name)| {
                        view! {
                            <div class="tab" 
                                on:click=move|_| {
                                    currently_selected_index.set(0);
                                    current_tab.set((i, tab_name.to_string()));
                                }
                                class:selected_tab=move|| current_tab.get().0 == i> 
                                    {tab_name}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <hr/>
                <ShopList current_tab variant_options current_variant currently_selected_index current_items_memo/>
                //<p inner_html={move|| data.get().description}/>
                <div class="shop_bottom_row">
                    <div class="money_wrapper"><span class="money">{move || cyberpunk_signal.read().money}</span></div>
                    <div class="shop_buttons_row">
                        <Show when=move|| {current_tab.get().1.as_str() == "Armor"}> 
                            Head armor: 
                            <input type="checkbox" prop:checked=move||head_armor.get() on:change=move|_| head_armor.update(|a| *a= !*a)/>
                        </Show>
                        <button on:click={let give_clone = give_item.clone(); move|_| give_clone(false)}>GIVE</button>
                        <button on:click={let give_clone = give_item.clone(); move|_| give_clone(true)}>BUY</button>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn ShopList(current_tab: RwSignal<(usize, String)>, variant_options: Memo<Vec<String>>, current_variant:RwSignal<String>, currently_selected_index: RwSignal<usize>, current_items_memo: Memo<Vec<ShopItemVisualData>>) -> AnyView {
    let currenty_selected_item = Memo::new(move |_| {
        current_items_memo.read().get(currently_selected_index.get()).expect("item to exist").clone()
    });
    view! {
        <div class="shop_content">
            <div class="name_list_wrapper">
                <Show when=move|| {variant_options.get().len() > 0}> 
                    <select 
                        class="shop_variant_select"
                        prop:value=move|| current_variant.get()
                        on:change:target=move |ev| {
                            let val = ev.target().value();
                            currently_selected_index.set(0);
                            current_variant.set(val);
                        }
                    >
                        <For 
                            each=move|| variant_options.get()
                            key=move|var| var.to_string()
                            children=move|var| {
                                view! {<option value=var.to_string()>{var.clone()}</option>}
                            }
                        />
                    </select>
                </Show>
                <div class="name_list">
                    <For each=move||{current_items_memo.get().into_iter().enumerate().collect::<Vec<_>>()}
                        key=move|(_, shop_item)| current_tab.get().1 + shop_item.name.clone().as_str() //category needs to be added to key, since some gear is also cyberware with the same name
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