use std::vec;
use leptos::prelude::*;
use cp_char_data::{char::GearType, gear::{get_map_key, GearData, Shoppable, ShoppableVisualData}};
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopTab {
    Weapons,
    Ammo,
    Armor,
    Cyberware,
    Drugs,
    Gear,
    Fashion,
    Hardware,
    Programs
}

impl ToString for ShopTab {
    fn to_string(&self) -> String {
        match self {
            ShopTab::Weapons => "Weapons".into(),
            ShopTab::Ammo => "Ammo".into(),
            ShopTab::Armor => "Armor".into(),
            ShopTab::Cyberware => "Cyberware".into(),
            ShopTab::Drugs => "Drugs".into(),
            ShopTab::Gear => "Gear".into(),
            ShopTab::Fashion => "Fashion".into(),
            ShopTab::Hardware => "Hardware".into(),
            ShopTab::Programs => "Programs".into(),
        }
    }
}

impl ShopTab {
    pub fn get_list() -> Vec<ShopTab> {
        vec![
            ShopTab::Weapons,
            ShopTab::Ammo,
            ShopTab::Armor,
            ShopTab::Cyberware,
            ShopTab::Drugs,
            ShopTab::Gear,
            ShopTab::Fashion,
            ShopTab::Hardware,
            ShopTab::Programs
        ]
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
    
    let current_tab: RwSignal<(usize, ShopTab)> = RwSignal::new((0, ShopTab::Weapons));
    let currently_selected_index: RwSignal<usize> = RwSignal::new(0);
    let head_armor = RwSignal::new(false);
    let variant_options = Memo::new(move |_| {
        if current_tab.read().1 == ShopTab::Ammo {
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
    
    let tabs = ShopTab::get_list();

    let current_items_memo: Memo<Vec<ShoppableVisualData>> = Memo::new(move |_| {
        let gear_data: GearData = use_context().expect("Expecting gear data existence");
        let mut list = match current_tab.get().1 {
            ShopTab::Weapons => gear_data.weapons.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Ammo => gear_data.ammunition.iter().filter(|ammo| ammo.caliber == current_variant.get()).map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Armor => gear_data.armor.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Cyberware => gear_data.cyberware.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Drugs => gear_data.drugs.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Gear => gear_data.items.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Fashion => gear_data.fashion.clone(),
            ShopTab::Hardware => gear_data.cyberdeck_hardware.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>(),
            ShopTab::Programs => gear_data.programs.iter().map(|val| ShoppableVisualData::from(val)).collect::<Vec<_>>()
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

    let add_gear_to_character = move |must_pay: bool, name: String, price: i32, gear_type: GearType| {
        if must_pay && !check_money_and_reduce(price) {
            return;
        }
        cyberpunk_signal.update(|c|c.add_gear(gear_type, name));
    };

    let give_item = move |must_pay: bool| {
        let current_item = currenty_selected_item.get();
        match current_tab.read().1 {
            ShopTab::Weapons => {
                let bought_item = gear_data.weapons.iter()
                    .find(|item| item.get_name() == &current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                if must_pay && !check_money_and_reduce(bought_item.get_price()) {
                    return;
                }
                cyberpunk_signal.update(|c|c.weapons.push(bought_item));
            },
            
            ShopTab::Ammo => {
                let bought_item = gear_data.ammunition.iter()
                    .find(|item| item.get_name() == &current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.get_price()) {
                    return;
                }
                cyberpunk_signal.update(|c|{
                    let amount = bought_item.only_one.then(||1).or(Some(10)).unwrap();
                    let changed_name = get_map_key(&bought_item);
                    if c.ammo.get_mut(&changed_name).and_then(|val| Some(*val += amount)).is_none() {
                        c.ammo.insert(changed_name, amount);
                    }
                });
            },
            
            ShopTab::Armor => {
                let mut bought_item = gear_data.armor.iter()
                    .find(|item| item.get_name() == &current_item.name)
                    .cloned()
                    .expect("expect item to exist");

                bought_item.sp_current = bought_item.sp;
                //shields do not go on head 
                if bought_item.type_field.as_str() == "armor" {
                    bought_item.head = head_armor.get();
                }
                if must_pay && !check_money_and_reduce(bought_item.get_price()) {
                    return;
                }
                cyberpunk_signal.update(|c|c.armors.push(bought_item));
            },
            
            ShopTab::Cyberware =>{
                let bought_item = gear_data.cyberware.iter()
                    .find(|item| item.get_name() == &current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                if must_pay && !check_money_and_reduce(bought_item.get_price()) {
                    return; 
                }
                cyberpunk_signal.update(|c|c.cyberware.push(bought_item));
            },
            
            ShopTab::Drugs => {
                let bought_item = gear_data.drugs.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                add_gear_to_character(must_pay, bought_item.name, bought_item.price, GearType::Drugs);
            },

            ShopTab::Gear => {
                let bought_item = gear_data.items.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                add_gear_to_character(must_pay, bought_item.name, bought_item.price, GearType::Gear);
            },
            
            ShopTab::Hardware => {
                let bought_item = gear_data.cyberdeck_hardware.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                add_gear_to_character(must_pay, bought_item.name, bought_item.price, GearType::Hardware);

            },
            
            ShopTab::Programs => {
                let bought_item = gear_data.programs.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                add_gear_to_character(must_pay, bought_item.name, bought_item.price, GearType::Programs);

            },

            ShopTab::Fashion => {
                let bought_item = gear_data.fashion.iter()
                    .find(|item| item.name == current_item.name)
                    .cloned()
                    .expect("expect item to exist");
                
                add_gear_to_character(must_pay, bought_item.name, bought_item.price, GearType::Fashion);
            }
            
        };
    };


    view! {
        <div class="modal" on:click=move |_| data.update(|data| data.hide())>
            <div class="modal_content" on:click=move |ev| { ev.stop_propagation();}>
                <div class="tabs_list">
                    {tabs.into_iter().enumerate().map(|(i, tab_enum)| {
                        view! {
                            <div class="tab" 
                                on:click=move|_| {
                                    currently_selected_index.set(0);
                                    current_tab.set((i, tab_enum));
                                }
                                class:selected_tab=move|| current_tab.get().0 == i> 
                                    {tab_enum.to_string()}
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
                        <Show when=move|| {current_tab.get().1 == ShopTab::Armor}> 
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
pub fn ShopList(current_tab: RwSignal<(usize, ShopTab)>, variant_options: Memo<Vec<String>>, current_variant:RwSignal<String>, currently_selected_index: RwSignal<usize>, current_items_memo: Memo<Vec<ShoppableVisualData>>) -> AnyView {
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
                        key=move|(_, shop_item)| current_tab.get().1.to_string() + shop_item.name.clone().as_str() //category needs to be added to key, since some gear is also cyberware with the same name
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

