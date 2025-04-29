use indexmap::IndexMap;
use leptos::prelude::*;
use std::cmp::{max, min};
use crate::gear::*;
use crate::icon_views::{AddIcon, RemoveIcon};
use crate::help::get_char_signal_from_ctx;
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
        <div class="weapon_view">
            <span class="weapon_name">{move|| item_memo.get().name.clone()}</span>
            <span class="weapon_bonus" 
                class:has_penalty=move||has_penalty()
            >
                {move|| get_skill_value.get() + weapon_bonus()}
            </span>
            <span class="weapon_rof">rof {move|| item_memo.get().weapon_data.rof.clone()}</span>
            <span class="weapon_damage">{move|| item_memo.get().weapon_data.damage.clone()}</span>
            <div class="weapon_buttons">
                <button
                    on:click=move|_| char_signal.update(|c|{
                        c.weapons.remove(index);
                    })>
                    X
                </button>
                <Show when=move|| has_ammo.get()>
                    <button
                        on:click=move|_| show_ammo_select_signal.update(|val| *val = !*val)>
                        AMMO
                    </button>
                </Show>
                <button
                    class:selected_tab=move || weapon_bonus() != 0
                    on:click=move|_| {char_signal.update(|c| {
                        c.weapons.get_mut(index).and_then(|weap: &mut Weapon| {
                            if weap.weapon_data.bonus.is_some() {
                                weap.weapon_data.bonus = None;
                            }
                            else {
                                weap.weapon_data.bonus = Some(1);
                            }
                            Some(weap)
                        });
                    })}
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
        .into_iter().enumerate()
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
                    prop:value=move || {char_signal.read().current_armor_head.or(Some(100)).unwrap()}
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
                    <option value=move||format!("{}", 100)>No Armor equipped</option>
                    <For 
                        each=move|| head_armors.get()
                        key=move|(_, armor)| armor.name.clone()
                        children=move|(index, armor)| {
                            view! {<option value=index.to_string()>{armor.name.clone()}</option>}
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
                    prop:value=move || {char_signal.read().current_armor_body.or(Some(100)).unwrap()}
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
                    <option value=move||format!("{}", 100)>No Armor equipped</option>
                    <For 
                        each=move|| body_armors.get()
                        key=move|(_, armor)| armor.name.clone()
                        children=move|(index, armor)| {
                            view! {<option value=index.to_string()>{armor.name.clone()}</option>}
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
    view! {
        <div class="gear_list">
            <For each=move|| {char_signal.read().cyberware.clone().into_iter().enumerate().collect::<Vec<_>>()}
                key=move|(_, cyber)| cyber.name.to_lowercase().clone()
                children=move|(indx, cyber)| {
                    view! {
                        <div class="gear_view">
                            <span>{move || cyber.name.to_string()}</span>
                            <button on:click=move|_|{char_signal.write().cyberware.remove(indx);}>
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
                    view! {
                        <div class="gear_view">
                            <span>
                                {move || char_signal.read().gear.get(&key).expect("expecting item amount to exist").to_string()}x
                                {move || find_item.clone().name.to_string()}
                            </span>
                            <button on:click=move|_|{ 
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
            <For each=move|| {char_signal.read().armors.clone().into_iter().enumerate().collect::<Vec<_>>()}
                key=move|(_, armor)| armor.name.to_lowercase().clone()
                children=move|(indx, armor)| {
                    view! {
                        <div class="gear_view">
                            <span>{move || format!(
                                "{} {}", 
                                armor.name.to_string(), 
                                armor.armor_data.head
                                    .then(||"(Head)")
                                    .or(Some("(Body)"))
                                    .unwrap()
                            )}</span>
                            <button on:click=move|_|{char_signal.write().armors.remove(indx);}>
                                X
                            </button>
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
                    view! {
                        <div class="gear_view">
                            <span>
                                {move || char_signal.read().ammo.get(&key).expect("expecting item amount to exist").to_string()}x
                                {move || find_item.clone().name.to_string()}
                            </span>
                            <button on:click=move|_|{ 
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