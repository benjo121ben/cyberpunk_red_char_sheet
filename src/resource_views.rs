use leptos::prelude::*;
use leptos::logging::log;
use std::cmp::{min, max};

use crate::gear::{get_map_key, GearData, Weapon, WeaponAmmoData};
use crate::help::get_char_signal_from_ctx;
use crate::icon_views::{AddIcon, RemoveIcon};


#[component]
pub fn AmmoViewRadial(count: Memo<i32>) -> AnyView {
    let check_visibility = move |nr| {
        if count.get() >= nr { "hidden"  } else {"visible"}
    };

    view! {
        <svg id="eHRlrbPxCUi1" class="ammo_svg" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 300 300" shape-rendering="geometricPrecision" text-rendering="geometricPrecision" project-id="95c5d48746bb4f409673ec40a2ef35d9" export-id="cf33590d4c6f4f0cbbde14f1333dbd2b" cached="false">
            <ellipse rx="50" ry="50" transform="matrix(-3 0 0-3.000001 150 149.765748)" fill="#d2dbed" stroke-width="0"/>
            <ellipse class="ammo_bullet" rx="50" ry="50" transform="matrix(-2.396996 0 0-2.396989 150 149.765748)" fill="#1da8df" stroke-width="0"/>
            <path d="M150,114.307486v90.338036" transform="matrix(3 0 0 3.000001-300-328.792324)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(0 3-3.000001 0 628.429671-300.362653)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(2.12132 2.12132-2.121321 2.121321 170.102813-506.86157)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <path d="M150,114.307486v90.338036" transform="matrix(2.12132-2.12132 2.121321 2.121321-506.498917 129.534534)" fill="none" stroke="#d2dbed" stroke-width="5" stroke-linecap="round"/>
            <ellipse rx="50" ry="50" transform="matrix(-1.153015 0 0-1.15301 150 149.765748)" fill="#d2dbed" stroke-width="0"/>
            <path style:visibility=move|| {check_visibility(8)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(-3 0 0 3.000001 596.127444-332.332035)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(7)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0-3 3.000001 0-325.377493 609.896205)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(6)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0 3 3.000001 0-324.163531-302.890133)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(5)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(-3 0 0-3.000001 604.018196 629.678837)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(4)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(3 0 0-3.000001-300.376306 632.245137)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(3)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0 3-3.000001 0 633.875226-304.98783)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(2)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(0-3-3.000001 0 629.019378 594.443883)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
            <path style:visibility=move|| {check_visibility(1)} d="M181.939319,127.537185L159.776806,150L150,144.441828v-24.786675l5.069412-5.34767l26.869905,13.229699" transform="matrix(3 0 0 3.000001-303.411211-329.382031)" fill="#d2dbed" stroke="#d2dbed" stroke-width="0.6"/>
        </svg> 

    }.into_any()
}

#[component]
pub fn HealthView() -> AnyView {
    let char_signal = get_char_signal_from_ctx();
    let get_max_health = move || char_signal.read().calc_max_health();
    let get_current_health = move || char_signal.read().hp_current;
    
    view! {
        <div class="health_border" style:grid-template-columns=move || {format!("repeat({}, 1fr)", get_max_health())}>
            <Show when=move || {get_current_health() > 0}>
                <div 
                    class="health_bar" 
                    style:grid-column=move || {format!("span {}", get_current_health())}
                />
            </Show>
            
        </div>    
        <div class="health_text">
            {move || format!("{}/{}", get_current_health(), get_max_health()) }
        </div>    
    }.into_any()
}

#[component]
pub fn AmmoViewLinear(count: Memo<i32>, max: Memo<i32>, weapon_index: usize, show_ammo_select: RwSignal<bool>) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let gear_data: GearData = use_context().expect("AmmoViewLinear: gear_data should exist at this point"); 

    let weapon_ammo_data_memo = Memo::new(move|_| {
        let weapon = char_signal.read().weapons.get(weapon_index).cloned().expect("expecting weapon to exist");
        weapon
            .weapon_data
            .ammo.clone()
            .expect("expecting ammo to be present").clone()
    });

    let get_ammo_gear_data = move |key: &String| {
        gear_data.ammunition.iter().find(|ammo| get_map_key(*ammo) == *key).cloned().expect("ammo should exist in gear_data")
    };

    let need_to_add_current_ammo_option = Memo::new(move |_| { 
        let ammo_data = weapon_ammo_data_memo();
        
        if ammo_data.current_ammo_type.is_none() {
            return false;
        }

        let ammo_type = ammo_data.current_ammo_type.unwrap();
        char_signal.read().ammo.get(&ammo_type).is_none()
    });

    let get_ammo_gear_data_clone = get_ammo_gear_data.clone();
    let get_current_ammo_data_for_option = Memo::new(move |_| {
        let ammo_data = weapon_ammo_data_memo();     

        if ammo_data.current_ammo_type.is_none() {
            return Some(("no_ammo".to_string(), "".to_string()))
        }
        let ammo_type = ammo_data.current_ammo_type.expect("expect ammo to exist");
        let gear_data = get_ammo_gear_data_clone(&ammo_type);
        Some((ammo_type, gear_data.name))
    }); 

    let get_current_ammo_select_key =  Memo::new(move |_| {
        weapon_ammo_data_memo.get()
        .current_ammo_type
        .or(Some("no_ammo".to_string()))
        .unwrap()
    });

    let reload = move || {
        let mut ret = Ok(());
        char_signal.update(|cyberpunk|{
            let weapon = cyberpunk.weapons.get_mut(weapon_index).expect("expecting weapon to exist");
            
            if weapon.weapon_data.ammo.is_none() {
                ret = Err(());
                return;
            }
            let ammo_data: &mut WeaponAmmoData = weapon.weapon_data.ammo.as_mut().expect("expecting ammo to be present");
            
            if ammo_data.current_ammo_type.is_none() {
                ret = Err(());
                return;
            }        
            let current_ammo = ammo_data.current_ammo_type.clone().unwrap();
            
            //check if we still have ammo in the inventory
            let inventory_ammo = cyberpunk.ammo.get_mut(&current_ammo);
            if inventory_ammo.is_none() {
                ret = Err(());
                return;
            }
            let inventory_ammo = inventory_ammo.expect("inventory ammo cannot be none at this point");
            let clip_size = ammo_data.max;
            let refill_amount = std::cmp::min(*inventory_ammo, clip_size);
            
            ammo_data.value += refill_amount;
            *inventory_ammo -= refill_amount;
            log!("inv {}", *inventory_ammo);
            if *inventory_ammo <= 0 {
                cyberpunk.ammo.shift_remove(&current_ammo);
            }
            return;
        });
        ret
    };

    let ammo_options = Memo::new(move |_| {
        let weapon_calibers = weapon_ammo_data_memo().compatible_calibers;
            
        //filter available ammo by caliber
        char_signal.read().ammo.iter()
            .map(|(key, _)| (key.clone(), get_ammo_gear_data(key)))
            .filter(|(_, ammo)| weapon_calibers.contains(&ammo.caliber))
            .map(|(key, ammo)| (key, ammo.name.clone()))
            .collect::<Vec<_>>()
    });

    let swap_ammo = move|new_ammo_key: String| {
        char_signal.update(|c|{
            let weapon = c.weapons.get_mut(weapon_index).unwrap();
            let new_val = (new_ammo_key.as_str() != "no_ammo")
                .then_some(new_ammo_key);

            let ammo_data = 
                weapon
                .weapon_data
                .ammo.as_mut()
                .unwrap();
            

            //put current ammo back into inventory
            if ammo_data.current_ammo_type.is_some() {
                let current_ammo_key = ammo_data.current_ammo_type.clone().unwrap();
                let current_ammo_amount = ammo_data.value;
                if current_ammo_amount > 0 {
                    if c.ammo.get_mut(&current_ammo_key).and_then(|val| Some(*val += current_ammo_amount)).is_none() {
                        c.ammo.insert(current_ammo_key, current_ammo_amount);
                    }
                }
            }

            ammo_data.current_ammo_type = new_val;
            ammo_data.value = 0;
            
        });
        show_ammo_select.set(false);
        reload();
    };
    
    view! {
        <Show when=move||{count.get() > 0}>
            <div class="ammo_view_linear"
                on:click=move|ev| {
                    ev.stop_propagation();
                    char_signal.update(|c|{
                        c.weapons.get_mut(weapon_index).and_then(|weap: &mut Weapon|
                            weap
                                .weapon_data
                                .ammo.as_mut()
                                .and_then(|ammo_data: &mut WeaponAmmoData| {ammo_data.shoot(); Some(ammo_data)})
                        );
                    });
                }
            >
                <div 
                    class="linear_ammo_grid"
                    style:grid-template-columns=move || {format!("repeat({}, 1fr)", max.get())}
                >
                    <For each={move || 0..max.get()}
                        key=move|nr| nr.to_string()
                        children=move |nr| {
                            view! {
                                <ResourceBar threshold=nr current_resource_state=count/>
                            } 
                        }
                    />
                </div>
                <span class="ammo_text">
                    {move || count.get()} / {move || max.get()}
                </span>
            </div>
        </Show>
        <Show when=move||{count.get() <= 0 && !show_ammo_select.get()}>
            <button class="ammo_reload" 
                on:click=move |ev| {
                    ev.stop_propagation();
                    if reload().is_err() {
                        show_ammo_select.set(true)
                    }
                }
            >
                RELOAD
            </button>
        </Show>
        <Show when=move||{show_ammo_select.get()}>
            <select class="ammo_select"
                on:click=move|ev| {
                    ev.stop_propagation();
                }
                on:change:target=move|ev| {
                    let new_ammo_key = ev.target().value();
                    swap_ammo(new_ammo_key)
                }
            >
                <option 
                    selected=move||{
                        get_current_ammo_select_key() == "no_ammo".to_string()
                    }
                    value=move||format!("no_ammo")
                >
                    No Ammo
                </option>
                <Show when=move|| need_to_add_current_ammo_option()>
                    <option 
                        value=move|| {
                            get_current_ammo_data_for_option()
                            .expect("expecting current ammo data to exist")
                            .0.clone()
                        }
                        selected=move||{
                            get_current_ammo_select_key() == get_current_ammo_data_for_option()
                                .expect("expecting current ammo data to exist")
                                .0.to_string()
                        }
                    >
                        {move || get_current_ammo_data_for_option().expect("expecting current ammo data to exist").1.clone()}
                    </option>
                </Show>
                <For 
                    each=move|| ammo_options()
                    key=move|(key, _)| key.clone()
                    children=move|(key, name)| {
                        let key_clone = key.clone();
                        view! {
                            <option 
                                selected=move||{
                                    get_current_ammo_select_key() == key.to_string()
                                }
                                value=key_clone.to_string()>{name.clone()}
                            </option>
                        }
                    }
                />
            </select>
        </Show>
    }
}


#[component]
pub fn ArmorBar(nr: i32, head: bool) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let get_current_sp = Memo::new(move |_| {
        char_signal.with(|cyberpunk| {
            let armor = if head {cyberpunk.get_current_head_armor()} else {cyberpunk.get_current_body_armor()};
            armor.map(|a| a.armor_data.sp_current).or(Some(0)).unwrap()
        })
    });
    let additional_class = head.then(|| "head_armor_bar").or(Some("")).unwrap().to_string();
    view! {
        <ResourceBar threshold=nr current_resource_state=get_current_sp opt_additional_classes=additional_class/>
    }
}

#[component]
pub fn ResourceBar(threshold: i32, current_resource_state: Memo<i32>, #[prop(optional)]opt_additional_classes:Option<String>) -> impl IntoView {
    let additional_classes = opt_additional_classes.or(Some("".to_string())).unwrap();
    let classes = additional_classes + " resource_bar";
    view! {
        <div class=classes class=("resource_bar_empty", move||current_resource_state.get() <= threshold)/>
    }
}

#[component]
pub fn ArmorView(head: bool) -> AnyView {
    let char_signal = get_char_signal_from_ctx();
    let armor_memo = Memo::new(move |_| {
        char_signal.with(|cyberpunk| {
            if head {cyberpunk.get_current_head_armor().cloned()} else {cyberpunk.get_current_body_armor().cloned()}
        })
    });
    let get_current_sp = Memo::new(move |_| {
        let armor = armor_memo.get();
        armor.map(|a| a.armor_data.sp_current).or(Some(0)).unwrap()
    });
    let get_max_sp = Memo::new(move |_| {
        let armor = armor_memo.get();
        armor.map(|a| a.armor_data.sp + a.armor_data.bonus.or(Some(0)).unwrap()).or(Some(0)).unwrap()
    });

    let ablate_repair_armor = move |amount: i32| {
        char_signal.update(|c| {
            let mut_armor = if head {
                c.get_current_head_armor_mut()
            } 
            else {
                c.get_current_body_armor_mut()
            };
            mut_armor.and_then(|armor| {
                let max_sp = armor.armor_data.sp + armor.armor_data.bonus.or(Some(0)).unwrap();
                armor.armor_data.sp_current = min(max(armor.armor_data.sp_current + amount, 0), max_sp);
                Some(armor)
            });
        });
    };

    view! {
        <div class="flex_col">
            <div class="armor_row"
                on:click=move|_| {let change_armor = ablate_repair_armor; change_armor(1)}
                on:contextmenu=move|_| {let change_armor = ablate_repair_armor; change_armor(-1)}
                style:grid-template-columns=move || {format!("repeat({}, 1fr)", get_max_sp())}
            >
                <For each={move || 0..get_max_sp()}
                    key=move|nr| nr.to_string()
                    children=move |nr| {
                        view! {<ArmorBar nr head/>} 
                    }
                />
            </div>
            <div class="armor_text">
                {move || {
                    let armor_tag = if head {"Head "} else {"Body "};
                    format!("{}{}/{}", armor_tag, get_current_sp(), get_max_sp())
                }}
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn HealthAdjustPopup(visible_signal: RwSignal<bool>) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let head_damage_signal = RwSignal::new(false);
    let change_health = move |amount: i32| {
        let negative_amount = -amount;
        char_signal.update(|c| {
            c.change_health_with_armor(head_damage_signal.get(), negative_amount);
        });
    };

    view! {
        <dialog class="health_popup">
            <div class="flex_row">
                <RemoveIcon on:click=move|_| {char_signal.write().change_health_without_armor(-1);}/>
                <AddIcon on:click=move|_| {char_signal.write().change_health_without_armor(1);}/>
            </div>
            <div class="flex_row">
                Head
                <input type="checkbox" on:change=move|_| head_damage_signal.update(|a| *a= !*a)/>
                <input autofocus 
                    class="health_change_input" 
                    placeholder="damage"
                    type="number" 
                    on:change=move|ev| {
                        let change_health_clone = change_health;
                        match event_target_value(&ev).parse::<i32>() {
                            Ok(number) => {
                                change_health_clone(number);
                            },
                            Err(_) => {},
                        };
                        visible_signal.set(false);
                    }
                /> 
            </div>
        </dialog>
    }
}

#[component]
pub fn MoneyView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let input_mode = RwSignal::new(false);
    let (money_getter, money_setter) = slice!(char_signal.money);
    view! {
        <Show
            when=move || !input_mode.get()
        >
            <span class="money"
                on:click=move|_| input_mode.set(true)
            >
                {move|| money_getter()}
            </span>
        </Show>
        <Show
            when=move || input_mode.get()
        >
            <input class="money_input" placeholder="set" prop:value=move||money_getter() on:change=move |ev| {
                match event_target_value(&ev).parse() {
                    Ok(number) => {
                        money_setter(number);
                        input_mode.set(false);
                    },
                    Err(_) => {},
                }
            }/>
            <input class="money_input" placeholder="add" on:change=move |ev| {
                match event_target_value(&ev).parse::<i32>() {
                    Ok(number) => {
                        let current_money = money_getter();
                        money_setter(current_money + number);
                        input_mode.set(false);
                    },
                    Err(_) => {},
                }
            }/>
        </Show>
    }
}
