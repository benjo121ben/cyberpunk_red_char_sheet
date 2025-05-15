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
    let range_table_signal: RwSignal<RangeType> = use_context().expect("expect range table to be set");

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

    let weapon_bonus = move || {
        let weapon = item_memo.get();
        let quality_bonus = (weapon.weapon_data.quality == ItemQuality::Excellent) as i32;
        let smart_bonus: i32 = weapon.weapon_data.attachments.contains(&"Smart".to_string()) as i32;
        quality_bonus + smart_bonus
    };

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
                    data.title = item_memo().get_name().clone();
                    data.description = item_memo().get_description().clone();
                    data.show();
                });
                show_weapon_name_input.set(false);
            }
            on:mouseover=move |_| {
                range_table_signal.set(item_memo.get().weapon_data.weapontype)
            }
            on:mouseleave=move |_| {
                range_table_signal.set(RangeType::None)
            }
        >
            <Show when=move||!show_weapon_name_input.get()>
                <span class="weapon_name"
                    on:click=move|ev| {
                        ev.stop_propagation();
                        show_weapon_name_input.set(true)
                    }
                >
                    {move|| item_memo.get().get_name().clone()}
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
                    class:selected_tab=move || item_memo.get().weapon_data.quality == ItemQuality::Excellent
                    on:click=move|ev| {
                        ev.stop_propagation();
                        char_signal.update(|c| {
                            c.weapons.get_mut(index).and_then(|weap: &mut Weapon| {
                                weap.weapon_data.quality = match weap.weapon_data.quality {
                                    ItemQuality::Average => ItemQuality::Excellent,
                                    ItemQuality::Excellent => ItemQuality::Poor,
                                    ItemQuality::Poor => ItemQuality::Average,
                                };
                                Some(weap)
                            });
                        })
                    }
                >
                    {move || match char_signal.read().weapons.get(index).unwrap().weapon_data.quality {
                        ItemQuality::Average => "A",
                        ItemQuality::Excellent => "E",
                        ItemQuality::Poor => "P",
                    }.to_string()}
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
            <WeaponAttachmentView index/>
        </div>
    }
}

#[component]
pub fn WeaponAttachmentView(index:usize) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let weapon_memo = Memo::new(move|_| {
        char_signal.read().weapons.get(index).unwrap().clone()
    });
    view!{
        <div class="attachment_view">
            <For each=move|| weapon_memo.get().weapon_data.attachments.clone()
                key=move|attachment| attachment.clone()
                children=move|attachment| {
                    view! {
                        <span>{move || attachment.clone()}</span>
                    }
                }
            />
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
        .filter(|(_, armor)|armor.head)
        .collect::<Vec<_>>())
    ;
    let body_armors = Memo::new(move |_| char_signal.read()
        .armors
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, armor)|!armor.head)
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
                let max_sp = armor.sp;
                armor.sp_current = min(max(armor.sp_current + amount, 0), max_sp);
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
                        key=move|(_, armor)| armor.get_name().clone()
                        children=move|(index, armor)| {
                            view! {
                                <option 
                                    selected=move || current_head_armor_val_memo() == index.to_string()
                                    value=index.to_string()>
                                    {armor.get_name().clone()}
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
                        key=move|(_, armor)| armor.get_name().clone()
                        children=move|(index, armor)| {
                            view! {
                                <option 
                                    selected=move || current_body_armor_val_memo() == index.to_string()
                                    value=index.to_string()>
                                    {armor.get_name().clone()}
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
                                data.title = cyber.get_name().clone();
                                data.description = cyber.get_description().clone();
                                data.show();
                            })
                        >
                            <span>{move || cyber_memo.get().get_name().to_string()}</span>
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
                            .bonus.clone()
                            .or(Some(0))
                            .unwrap()
                    };
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = armor_memo.get().get_name().clone();
                                data.description = armor_memo.get().get_description().clone();
                                data.show();
                            })
                        >
                            <span>{move || format!(
                                "{} {}", 
                                armor_memo.get().get_name().to_string(), 
                                armor_memo.get().head
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
                                                if armor.bonus.is_some() {
                                                    armor.sp_current = max(armor.sp_current -1, 0);
                                                    armor.bonus = None;
                                                }
                                                else {
                                                    armor.sp_current += 1;
                                                    armor.bonus = Some(1);
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
                        let changed_name = get_map_key_from_name(find_item.get_name());
                        changed_name == key
                    }).cloned().expect("expecting item to exist");
                    let key_clone = key.clone();
                    let find_item_clone = find_item.clone();
                    view! {
                        <div class="gear_view"
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item_clone.get_name().clone();
                                data.description = find_item_clone.get_description().clone();
                                data.show();
                            })
                        >
                            <span>
                                {move || char_signal.read().ammo.get(&key).expect("expecting item amount to exist").to_string()}x
                                {move || find_item.clone().get_name().to_string()}
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

fn get_row_table(weapon_type: RangeType) -> Vec<i32> {
    match weapon_type {
        RangeType::Pistol => vec![13, 15, 20, 25, 30, 30],
        RangeType::Shotgun => vec![13, 15, 20, 25, 30, 35],
        RangeType::Assault => vec![17, 16, 15, 13, 15, 20, 25, 30],
        RangeType::Sniper => vec![30, 25, 25, 20, 15, 16, 17, 20],
        RangeType::SMG => vec![15, 13, 15, 20, 25, 25, 30],
        RangeType::Bow => vec![15, 13, 15, 17, 20, 22],
        RangeType::Grenade => vec![16, 15, 15, 17, 20, 22, 25],
        RangeType::Rocket => vec![17, 16, 15, 15, 20, 20, 25, 30],
        RangeType::None => vec![],
        RangeType::Melee => vec![],
    }
}

#[component]
pub fn RangeTable() -> impl IntoView {
    let range_table_signal: RwSignal<RangeType> = use_context().expect("expect range table to be set");
    let distance_vec = vec![(6, 3), (12, 6), (25, 12), (50, 25), (100, 50), (200, 100), (400, 200), (800, 400)];
    let value_list = move || get_row_table(range_table_signal.get());

    view! {
        <Show when=move || {value_list().len() > 0}>
            <table>
                <tr>
                    {
                        let vector_clone = distance_vec.clone();
                        move || {
                            vector_clone.clone().into_iter().map(|(meter, squares)|{
                            view! {
                                <th>
                                    {format!("{} ({})", meter, squares)}
                                </th>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </tr>
                <tr>
                    <For each=move|| 0..8
                        key=move|indx| indx.to_string()
                        children=move |indx| {
                            let value = move || {
                                value_list()
                                .get(indx)
                                .map(|val| val.to_string())
                                .or(Some("N/A".to_string()))
                                .unwrap()
                            };
                            view!{
                                <td class:dark_bg=move||value().as_str() == "N/A">{move || value()}</td>
                            }
                        }
                    />
                </tr>
            </table>

        </Show>
    }
    
}