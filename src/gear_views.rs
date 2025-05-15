use indexmap::IndexMap;
use leptos::logging::log;
use leptos::prelude::*;
use std::cmp::{max, min};
use cp_char_data::gear::*;
use crate::icon_views::{AddIcon, RemoveIcon};
use crate::help::get_char_signal_from_ctx;
use crate::info_modal_view::SimpleModalData;
use crate::resource_views::AmmoViewLinear;

#[component]
pub fn GearView() -> impl IntoView {
    view! {
        <div class="gear_list_wrapper">
            <AllWeaponsView/>
            <hr style="width:90%"/>
            <AllItemsView/>
        </div>
    }
}

#[component]
pub fn AllWeaponsView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    view! {
        <div class="gear_list">
            <For each=move|| 0..char_signal.read().weapons.len()
                key=move|index| index.to_string()
                children=move|index| {
                    view! {
                        <SingleWeaponView index/>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn SingleWeaponView(index:usize) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let show_ammo_select_signal = RwSignal::new(false);
    let show_weapon_name_input = RwSignal::new(false);
    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("allitemsview: simple modal should exist");

    let item_memo = Memo::new(move|_| {
        char_signal.read().weapons.get(index).unwrap().clone()
    });
    let skill_memo = Memo::new(move |_| char_signal.with(|c| c.skills.get(&item_memo.read().weapon_data.skill.clone()).expect("expect skill to exist in its own list").clone()));
    let get_skill_value = Memo::new(move |_| {
        let skill = skill_memo.get();
        let stat_nr = char_signal.with(|char| char.get_stat(&skill.stat.clone()).0);
        stat_nr + skill.nr
    });

    let has_penalty = Memo::new(move |_| {
        let stat = skill_memo.get().stat;
        char_signal.read().get_stat(&stat).1
    });

    let weapon_bonus = move || item_memo.get().weapon_data.bonus.or(Some(0)).unwrap();

    let has_ammo = Memo::new(move |_| item_memo.get().weapon_data.ammo.is_some());

    let ammo_max_memo = Memo::new(move |_| {
        item_memo.get()
            .weapon_data
            .ammo
            .and_then(|ammo_data|Some(ammo_data.max))
            .or(Some(0))
            .unwrap()
    });

    let ammo_memo = Memo::new(move |_| {
        item_memo.get()
            .weapon_data
            .ammo
            .and_then(|ammo_data|Some(ammo_data.value))
            .or(Some(0))
            .unwrap()
    });

    view!{
        <div class="weapon_view"
            on:click=move|_| {
                simple_modal_signal.update(|data| {
                    data.title = item_memo().name.clone();
                    data.description = item_memo().description.clone();
                    data.show();
                });
                show_weapon_name_input.set(false);
            }
        >
            <Show when=move||!show_weapon_name_input.get()>
                <span class="weapon_name"
                    on:click=move|ev| {
                        ev.stop_propagation();
                        show_weapon_name_input.set(true)
                    }
                >
                    {move|| {
                        let item = item_memo.get();
                        (item.personalized_name != "")
                            .then(|| item.personalized_name)
                            .or(Some(item.name))
                            .clone()
                            .unwrap()
                    }}
                </span>
            </Show>
            <Show when=move||show_weapon_name_input.get()>
                <input class="weapon_name_input" 
                    prop:value=move||item_memo.get().personalized_name.clone()
                    on:click=move|ev| ev.stop_propagation()
                    on:change=move|ev| {
                        char_signal.update(|cyberpunk| cyberpunk.weapons.get_mut(index).unwrap().personalized_name = event_target_value(&ev));
                        show_weapon_name_input.set(false);
                    }
                />
            </Show>
            <div class="weapon_center_div_wrapper">
                <div class="weapon_center_div">
                    <span class="weapon_bonus" 
                        class:has_penalty=move||has_penalty()
                    >
                        {move|| get_skill_value.get() + weapon_bonus()}
                    </span>
                    <span class="weapon_rof">rof {move|| item_memo.get().weapon_data.rof.clone()}</span>
                    <span class="weapon_damage">{move|| item_memo.get().weapon_data.damage.clone()}</span>
                </div>
            </div>
            <div class="gear_buttons">
                <button
                    on:click=move|ev| char_signal.update(|c|{
                        ev.stop_propagation();
                        c.weapons.remove(index);
                    })>
                    X
                </button>
                <Show when=move|| has_ammo.get()>
                    <button
                        on:click=move|ev| {
                            ev.stop_propagation();
                            show_ammo_select_signal.update(|val| *val = !*val)
                        }>
                        AMMO
                    </button>
                </Show>
                <button
                    class:selected_tab=move || weapon_bonus() != 0
                    on:click=move|ev| {
                        ev.stop_propagation();
                        char_signal.update(|c| {
                            c.weapons.get_mut(index).and_then(|weap: &mut Weapon| {
                                if weap.weapon_data.bonus.is_some() {
                                    weap.weapon_data.bonus = None;
                                }
                                else {
                                    weap.weapon_data.bonus = Some(1);
                                }
                                Some(weap)
                            });
                        })
                    }
                >
                    +1
                </button>
            </div>
            <Show when=move|| has_ammo.get()>
                <div class="weapon_ammo">
                    <AmmoViewLinear 
                        count=ammo_memo 
                        max=ammo_max_memo
                        weapon_index=index
                        show_ammo_select=show_ammo_select_signal
                    />
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn ArmorSelectionView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();

    let head_armors = Memo::new(move |_| char_signal.read()
        .armors
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, armor)|armor.armor_data.head)
        .collect::<Vec<_>>())
    ;
    let body_armors = Memo::new(move |_| char_signal.read()
        .armors
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, armor)|!armor.armor_data.head)
        .collect::<Vec<_>>())
    ;

    let ablate_repair_armor = move |head: bool, amount: i32| {
        char_signal.update(|c| {
            let mut_armor = if head {
                c.get_current_head_armor_mut()
            } 
            else {
                c.get_current_body_armor_mut()
            };
            mut_armor.and_then(|armor| {
                let max_sp = armor.armor_data.sp;
                armor.armor_data.sp_current = min(max(armor.armor_data.sp_current + amount, 0), max_sp);
                Some(armor)
            });
        });
    };

    let current_head_armor_val_memo = Memo::new(move |_| {char_signal.read().current_armor_head.or(Some(100)).unwrap().to_string()});
    let current_body_armor_val_memo = Memo::new(move |_| {char_signal.read().current_armor_body.or(Some(100)).unwrap().to_string()});

    view! {
        <div class="armor_selection">
            <div class="single_armor_selection">
                <Show when=move|| char_signal.read().current_armor_head.is_some()>
                    <RemoveIcon
                        on:click={
                            move|ev| {let change_armor = ablate_repair_armor; ev.stop_propagation(); change_armor(true, -1)}
                        }
                    />
                    <AddIcon
                        on:click={
                            move|ev| {let change_armor = ablate_repair_armor; ev.stop_propagation(); change_armor(true, 1)}
                        }
                    />
                </Show>
                <select 
                    class="head_armor_select"
                    on:change:target=move |ev| {
                        let val = ev.target().value().parse().unwrap();
                        if val == 100 {
                            char_signal.update(|c| c.current_armor_head =None);
                        }
                        else {
                            char_signal.update(|c| c.current_armor_head = Some(val));

                        }
                    }
                >
                    <option 
                        selected=move || current_head_armor_val_memo() == "100"
                        value=move||format!("{}", 100)>
                        No Armor equipped
                    </option>

                    <For 
                        each=move|| head_armors.get()
                        key=move|(_, armor)| armor.name.clone()
                        children=move|(index, armor)| {
                            view! {
                                <option 
                                    selected=move || current_head_armor_val_memo() == index.to_string()
                                    value=index.to_string()>
                                    {armor.name.clone()}
                                </option>}
                        }
                    />
                </select>
            </div>
            <div class="single_armor_selection">
                <Show when=move|| char_signal.read().current_armor_body.is_some()>
                    <RemoveIcon
                        on:click={
                            move|ev| {let change_armor = ablate_repair_armor; ev.stop_propagation(); change_armor(false, -1)}
                        }
                    />
                    <AddIcon
                        on:click={
                            move|ev| {let change_armor = ablate_repair_armor; ev.stop_propagation(); change_armor(false, 1)}
                        }
                    />
                </Show>
                <select 
                    class="body_armor_select"
                    on:change:target=move |ev| {
                        let val = ev.target().value().parse().unwrap();
                        if val == 100 {
                            char_signal.update(|c| c.current_armor_body =None);
                        }
                        else {
                            char_signal.update(|c| c.current_armor_body = Some(val));
                        }
                    }
                >
                    <option 
                        selected=move || current_body_armor_val_memo() == "100".to_string()
                        value=move||format!("{}", 100)>
                        No Armor equipped
                    </option>
                    <For 
                        each=move|| body_armors.get()
                        key=move|(_, armor)| armor.name.clone()
                        children=move|(index, armor)| {
                            view! {
                                <option 
                                    selected=move || current_body_armor_val_memo() == index.to_string()
                                    value=index.to_string()>
                                    {armor.name.clone()}
                                </option>}
                        }
                    />
                </select>
            </div>
        </div>
    }
}

#[component]
pub fn AllItemsView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let gear_data: GearData = use_context().expect("Gear Data should exist");
    let reduce_or_remove = move |item_map: &mut IndexMap<String, i32>, key: &String| {
        match item_map.get_mut(key) {
            Some(value) => {
                if *value <= 1 {
                    item_map.shift_remove(key);
                }
                else {
                    *value -= 1;
                }
            },
            None => todo!(),
        }
    };

    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("allitemsview: simple modal should exist");

    view! {
        <div class="gear_list">
            <For each=move|| {0..char_signal.read().cyberware.len()}
                key=move|indx| indx.to_string()
                children=move|indx| {
                    let cyber_memo = Memo::new(move|_| char_signal.read().cyberware.get(indx).expect("expect cyberware to exist in list").clone());
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                let cyber = cyber_memo.get();
                                data.title = cyber.name.clone();
                                data.description = cyber.description.clone();
                                data.show();
                            })
                        >
                            <span>{move || cyber_memo.get().name.to_string()}</span>
                            <button on:click=move|ev|{
                                ev.stop_propagation();
                                log!("rem index {}", indx);
                                char_signal.write().cyberware.remove(indx);
                            }>
                                X
                            </button>
                        </div>
                    }
                }
            />
            <hr style="width:90%"/>
            <For each=move|| {char_signal.read().gear.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = gear_data.items.iter().find(|find_item| {
                        let changed_name = find_item.name.to_lowercase().replace(" ", "_");
                        changed_name == key
                    }).cloned().expect("expecting item to exist");
                    let key_clone = key.clone();
                    let find_item_clone = find_item.clone();
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item_clone.name.clone();
                                data.description = find_item_clone.description.clone();
                                data.show();
                            })
                        >
                            <span>
                                {move || char_signal.read().gear.get(&key).expect("expecting item amount to exist").to_string()}x
                                {move || find_item.clone().name.to_string()}
                            </span>
                            <button on:click=move|ev|{ 
                                ev.stop_propagation();
                                let change_fn = reduce_or_remove.clone(); 
                                char_signal.update(|c| {
                                    change_fn(&mut c.gear, &key_clone)
                                })
                            }>
                                X
                            </button>
                        </div>
                    }
                }
            />

            <hr style="width:90%"/>
            <For each=move|| {0..char_signal.read().armors.len()}
                key=move|index| index.to_string()
                children=move|indx| {
                    let armor_memo = Memo::new(move|_| char_signal.read().armors.get(indx).expect("expect armor to exist").clone());
                    let armor_bonus = move || {
                        armor_memo.get()
                            .armor_data
                            .bonus.clone()
                            .or(Some(0))
                            .unwrap()
                    };
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = armor_memo.get().name.clone();
                                data.description = armor_memo.get().description.clone();
                                data.show();
                            })
                        >
                            <span>{move || format!(
                                "{} {}", 
                                armor_memo.get().name.to_string(), 
                                armor_memo.get().armor_data.head
                                    .then(||"(Head)")
                                    .or(Some("(Body)"))
                                    .unwrap()
                            )}</span>

                            <div class="gear_buttons">
                                <button on:click=move|ev|{
                                    ev.stop_propagation();
                                    log!("rem index {}", indx);
                                    char_signal.write().armors.remove(indx);
                                }>
                                    X
                                </button>
                                <button
                                    class:selected_tab=move || armor_bonus() != 0
                                    on:click=move|ev| {
                                        ev.stop_propagation();
                                        char_signal.update(|c| {
                                            c.armors.get_mut(indx).and_then(|armor: &mut Armor| {
                                                if armor.armor_data.bonus.is_some() {
                                                    armor.armor_data.sp_current = max(armor.armor_data.sp_current -1, 0);
                                                    armor.armor_data.bonus = None;
                                                }
                                                else {
                                                    armor.armor_data.sp_current += 1;
                                                    armor.armor_data.bonus = Some(1);
                                                }
                                                Some(armor)
                                            });
                                        })
                                    }
                                >
                                    +1
                                </button>
                            </div>
                        </div>
                    }
                }
            />
        
            <hr style="width:90%"/>
            <For each=move|| {char_signal.read().ammo.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = gear_data.ammunition.iter().find(|find_item| {
                        let changed_name = find_item.name.to_lowercase().replace(" ", "_");
                        changed_name == key
                    }).cloned().expect("expecting item to exist");
                    let key_clone = key.clone();
                    let find_item_clone = find_item.clone();
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item_clone.name.clone();
                                data.description = find_item_clone.description.clone();
                                data.show();
                            })
                        >
                            <span>
                                {move || char_signal.read().ammo.get(&key).expect("expecting item amount to exist").to_string()}x
                                {move || find_item.clone().name.to_string()}
                            </span>
                            <button on:click=move|ev|{ 
                                ev.stop_propagation();
                                let change_fn = reduce_or_remove.clone(); 
                                char_signal.update(|c| {
                                    change_fn(&mut c.ammo, &key_clone)
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