use leptos::prelude::*;
use crate::gear::*;
use crate::help::get_char_signal_from_ctx;
use crate::resource_views::AmmoView;

#[component]
pub fn GearView() -> impl IntoView {
    view! {
        <AllWeaponsView/>
    }
}

#[component]
pub fn AllWeaponsView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    view! {
        <For each=move|| 0..char_signal.read().weapons.len()
            key=move|index| index.to_string()
            children=move|index| {
                view! {
                    <SingleWeaponView index/>
                }
            }
        />
    }
}

#[component]
pub fn SingleWeaponView(index:usize) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let item_memo = Memo::new(move|_| {
        char_signal.read().weapons.get(index).unwrap().clone()
    });
    let skill_memo = Memo::new(move |_| char_signal.with(|c| c.skills.get(&item_memo.read().weapon_data.skill.clone()).expect("expect skill to exist in its own list").clone()));
    let get_skill_value = move || {
        let skill = skill_memo.get();
        char_signal.with(|char| char.get_stat(&skill.stat.clone())) + skill.nr
    };

    let has_ammo = Memo::new(move |_| item_memo.get().weapon_data.ammo.is_some());

    let ammo_memo = Memo::new(move |_| {
        char_signal.read().weapons.get(index)
            .cloned()
            .unwrap()
            .weapon_data
            .ammo
            .and_then(|ammo_data|Some(ammo_data.value))
            .or(Some(0))
            .unwrap()
    });
    view!{
        <div class="weapon_view">
            <div class="weapon_name">{move|| item_memo.get().name.clone()}</div>
            <div class="weapon_bonus">{move|| get_skill_value()}</div>
            <div class="weapon_damage">{move|| item_memo.get().weapon_data.damage.clone()}</div>
            <Show when=move|| has_ammo.get()>
                <AmmoView count=ammo_memo 
                    on:click=move|_| char_signal.update(|c|{
                        c.weapons.get_mut(index).and_then(|weap: &mut Weapon|
                            weap
                                .weapon_data
                                .ammo.as_mut()
                                .and_then(|ammo_data: &mut WeaponAmmoData| {ammo_data.shoot(); Some(ammo_data)})
                        );
                    })
                />
            </Show>
            <Show when=move|| !has_ammo.get()>
                <div/>
            </Show>
            <div class="weapon_buttons">
                <button
                    on:click=move|_| char_signal.update(|c|{
                        c.weapons.remove(index);
                    })>
                    X
                </button>
                <Show when=move|| has_ammo.get()>
                    <button
                        on:click=move|_| char_signal.update(|c|{
                            c.weapons.get_mut(index).and_then(|weap: &mut Weapon|
                                weap
                                    .weapon_data
                                    .ammo.as_mut()
                                    .and_then(|ammo_data: &mut WeaponAmmoData| {ammo_data.reload(); Some(ammo_data)})
                            );
                        })>
                        RELOAD
                    </button>
                </Show>
                <button
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
        .filter(|(index, armor)|armor.armor_data.head)
        .collect::<Vec<_>>())
    ;
    let body_armors = Memo::new(move |_| char_signal.read()
        .armors
        .clone()
        .into_iter().enumerate()
        .filter(|(index, armor)|!armor.armor_data.head)
        .collect::<Vec<_>>())
    ;
    view! {
        <div class="armor_selection">
            <select 
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
            <select 
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
    }
}
