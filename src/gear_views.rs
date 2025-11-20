use leptos::ev::MouseEvent;
use leptos::logging::log;
use leptos::prelude::*;
use std::cmp::{max, min};
use cp_char_data::gear::*;
use crate::icon_views::{AddIcon, RemoveIcon};
use crate::help::{get_char_signal_from_ctx, reduce_or_remove_items_in_map};
use crate::info_modal_view::SimpleModalData;
use crate::netrun_view::NetrunView;
use crate::resource_views::{AmmoViewLinear, ShieldView};
use crate::text_views::TabView;

#[component]
pub fn GearView() -> impl IntoView {
    let tab_index: RwSignal<usize> = RwSignal::new(0);
    view! {
        <div class="gear_list_wrapper">
            <TabView
                tabs_list=Memo::new(move|_|vec!["Gear".to_string(), "Netrunner".to_string(), "Fashion".to_string()])
                selected_tab_index=tab_index
            />
            <Show when=move||tab_index() == 0>
                <AllWeaponsView/>
                <hr style="width:90%"/>
                <AllItemsView/>
            </Show>
            <Show when=move||tab_index() == 1>
                <NetrunView/>
            </Show>
            <Show when=move||tab_index() == 2>
                <FashionView/>
            </Show>
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
    let skill_data_memo = Memo::new(move |_| char_signal.with(|c| c.get_full_skill_data(&item_memo.read().weapon_data.skill.clone())));
    let get_skill_value_memo = Memo::new(move |_| {
        skill_data_memo.read().1
    });

    let has_penalty_memo = Memo::new(move |_| {
        skill_data_memo.read().2
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
            .and_then(|ammo_data|Some(ammo_data.get_max_ammo()))
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
                        class:has_penalty=move||has_penalty_memo()
                    >
                        {move|| get_skill_value_memo.get() + weapon_bonus()}
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
    let gear_data: GearData = use_context().expect("expecting gear to exist (attachment)");
    let gear_signal = RwSignal::new(gear_data);

    let show_add_attachment_signal = RwSignal::new(false);
    let weapon_memo = Memo::new(move|_| {
        char_signal.read().weapons.get(index).unwrap().clone()
    });

    let has_mag = move || {
        weapon_memo.with(|weap| {
            weap.weapon_data.attachments.iter().find(|att| att.contains("mag")).is_some()
        })
    };

    let remove_attachment = move |shorthand: String | {
        char_signal.update(|cyber| {
            let weapon = cyber.weapons.get_mut(index).unwrap();
            for (i, att) in weapon.weapon_data.attachments.iter().enumerate() {
                if *att == shorthand {
                    weapon.weapon_data.attachments.remove(i);
                    break;
                }
            }
        });
    };

    let get_possible_attachments = move || {
        let weapon_data = weapon_memo.get().weapon_data;
        let mut ret_vec: Vec<Attachment> = vec![];
        if weapon_data.skill.as_str() == "melee_weapon" {
            return vec![];
        }
        gear_signal.with(|gear| {
            for attachment in gear.attachments.iter() {

                // only one mag
                if attachment.slot_type.is_some() && attachment.slot_type.clone().unwrap().as_str() == "mag" && has_mag() {
                    continue;
                }

                // cannot fit on weapon anymore (attachment slots)
                if attachment.slot_size > weapon_memo.get().get_free_attachment_slots(&gear) {
                    continue;
                }

                // same attachment only once
                if weapon_data.attachments.iter().find(|att| **att == attachment.shorthand).is_some() {
                    continue;
                }

                // is only for shoulder arms weapons
                if attachment.selector.only_shoulder_arms.is_some() && weapon_data.skill != "shoulder_arms" {
                    continue;
                }

                // weapon type is invalid
                let exclude_type = attachment.selector.exclude_type.clone();
                if exclude_type.is_some() && exclude_type.unwrap() == weapon_data.weapontype {
                    continue;
                }

                ret_vec.push(attachment.clone());
            }
        });
        return ret_vec;
    };

    view!{
        <div class="attachment_view">
            <For each=move|| 0..weapon_memo.get().weapon_data.attachments.len()
                key=move|i| i.to_string()
                children=move|i| {
                    let attachment = Memo::new(move|_| weapon_memo.get().weapon_data.attachments.get(i).cloned().expect("expecting index to be valid"));
                    let description = gear_signal.get()
                        .attachments.iter()
                        .find(|att| att.shorthand == attachment.get())
                        .cloned()
                        .expect("expecting data to exist")
                        .description;
                    view! {
                        <span 
                            title=move||description.clone()
                            on:contextmenu=move|ev|{ev.stop_propagation(); ev.prevent_default(); remove_attachment(attachment.get())}
                        >
                            {move || attachment.clone()}
                        </span>
                    }
                }
            />
            <Show 
                when=move||{ gear_signal.with(|gear_ref|weapon_memo.get().get_free_attachment_slots(gear_ref) > 0)}
            >
                <button on:click=move|ev|{ev.stop_propagation(); show_add_attachment_signal.update(|s| *s = !*s)}>+</button>
            </Show>
            <Show 
                when=move|| show_add_attachment_signal.get()
            >
                <select
                    on:change:target=move |ev| {
                        let val = ev.target().value();
                        char_signal.update(|punk| {
                            punk.weapons
                                .get_mut(index)
                                .expect("expecting weapon to exist | add attachment")
                                .weapon_data
                                .attachments
                                .push(val)
                            ;
                        });
                        show_add_attachment_signal.set(false);
                    }
                >
                    <option val="none">choose attachment</option>
                    <For
                        each=move|| get_possible_attachments()
                        key=move|attachment| attachment.name.clone()
                        children=move|attachment| {
                            view! {
                                <option value=move || attachment.shorthand.clone()>
                                    {move || attachment.name.clone()}
                                </option>
                            }
                        }
                    />
                </select>
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
    let range_table_signal: RwSignal<RangeType> = use_context().expect("expect range table to be set");
    let gear_data: GearData = use_context().expect("Gear Data should exist");

    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("allitemsview: simple modal should exist");

    view! {
        <div class="gear_list">
            <For each=move|| {char_signal.read().ammo.clone().into_iter().map(|(key, _)| key).collect::<Vec<_>>()}
                key=move|key| key.clone()
                children=move|key| {
                    let find_item = gear_data.ammunition.iter().find(|find_item| {
                        let changed_name = get_map_key_from_name(find_item.get_name());
                        changed_name == key
                    }).cloned().expect("expecting item to exist");
                    let key_clone = key.clone();
                    let find_item_clone = find_item.clone();
                    let find_item_clone2 = find_item.clone();
                    let display_text = Memo::new(move |_| format!("{}x{}", char_signal.read().ammo.get(&key).expect("expecting ammo item amount to exist"), find_item.clone().get_name()));
                    view! {
                        <SimpleItemView
                            display_text
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item_clone.get_name().clone();
                                data.description = find_item_clone.get_description().clone();
                                data.show();
                            })
                            on:mouseover=move |_| {
                                if find_item_clone2.caliber.as_str() == "grenade" {
                                    range_table_signal.set(RangeType::Grenade);
                                }
                            }
                            on:mouseleave=move |_| {
                                range_table_signal.set(RangeType::None)
                            }
                            on_x_click=move|_|{ 
                                char_signal.update(|c| {
                                    reduce_or_remove_items_in_map(&mut c.ammo, &key_clone)
                                })
                            }
                        />
                    }
                }
            />
            <hr style="width:90%"/>
            <For each=move|| {0..char_signal.read().cyberware.len()}
                key=move|indx| indx.to_string()
                children=move|indx| {
                    let cyber_memo = Memo::new(move|_| char_signal.read().cyberware.get(indx).expect("expect cyberware to exist in list").clone());
                    view! {
                        <SimpleItemView
                            display_text=move || cyber_memo.get().get_name().to_string()
                            on:click=move|_| simple_modal_signal.update(|data| {
                                let cyber = cyber_memo.get();
                                data.title = cyber.get_name().clone();
                                data.description = cyber.get_description().clone();
                                data.show();
                            })
                            on_x_click=move|_| {char_signal.write().cyberware.remove(indx);}
                        />
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
                        <SimpleItemView
                            display_text= move || format!("{}x{}", char_signal.read().gear.get(&key).expect("expecting gear item amount to exist"), find_item.clone().name.to_string())
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = find_item_clone.name.clone();
                                data.description = find_item_clone.description.clone();
                                data.show();
                            })
                            on_x_click=move|_| char_signal.update(|c| reduce_or_remove_items_in_map(&mut c.gear, &key_clone))
                        />
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
                        <div class="armor_gear_div">
                            <div class="gear_view"
                                on:click=move|_| simple_modal_signal.update(|data| {
                                    data.title = armor_memo.get().get_name().clone();
                                    data.description = armor_memo.get().get_description().clone();
                                    data.show();
                                })
                            >
                                <span>{move || format!(
                                    "{}{}", 
                                    armor_memo.get().get_name().to_string(), 
                                    {
                                        let armor = armor_memo.get();
                                        if armor.type_field.as_str() != "armor" {
                                            ""
                                        }    
                                        else if armor.head {
                                            " (Head)"
                                        }
                                        else {
                                            " (Body)"
                                        }
                                    }
                                )}</span>

                                <div class="gear_buttons">
                                    <button on:click=move|ev|{
                                        ev.stop_propagation();
                                        log!("rem index {}", indx);
                                        char_signal.write().armors.remove(indx);
                                    }>
                                        X
                                    </button>
                                    <Show when=move || armor_memo.get().type_field.as_str() == "armor">
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
                                    </Show>
                                </div>
                                <Show when=move || armor_memo.get().type_field.as_str() == "shield">
                                    <ShieldView armor_index=indx/>
                                </Show>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn FashionView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let simple_modal_signal: RwSignal<SimpleModalData> = use_context().expect("FashionView: simple modal should exist");
    let gear_data: GearData = use_context().expect("expecting gear to exist (FashionView)");
    view! {
        <div class="gear_list">
            <For each=move|| {char_signal.read().fashion.keys().cloned().collect::<Vec<_>>()}
                key=move|key| key.to_string()
                children=move|key| {
                    let key_clone: String = key.clone();
                    let find_item = gear_data.fashion.iter().find(|item| {
                        let changed_name = get_map_key_from_name(&item.name);
                        changed_name == *key
                    }).cloned().expect(&format!("expect fashion item to exist {}", key));
                    let title = find_item.name.clone();
                    let description = find_item.description.clone();
                    view! {
                        <SimpleItemView
                            display_text= move || format!("{}x {}", char_signal.read().fashion.get(&*key).expect("expecting fashion item amount to exist"), find_item.clone().name.to_string())
                            on:click=move|_| simple_modal_signal.update(|data| {
                                data.title = title.clone();
                                data.description = description.clone();
                                data.show();
                            })
                            on_x_click=move|_| char_signal.update(|c| reduce_or_remove_items_in_map(&mut c.gear, &key_clone))
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn SimpleItemView(
    display_text: impl Fn() -> String + Send + 'static,
    mut on_x_click: impl FnMut(MouseEvent) + 'static
) -> impl IntoView {
    view! {
        <div class="gear_view">
            <span>{display_text}</span>
            <button on:click=move|ev|{ 
                ev.stop_propagation();
                on_x_click(ev);
            }>
                X
            </button>
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