use leptos::prelude::*;
use leptos::logging::log;
use leptos::tachys::view;
use crate::gear::*;
use crate::help::get_char_signal_from_ctx;
use crate::resource_views::AmmoView;

#[component]
pub fn GearView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
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
                        c.weapons.get_mut(index)
                            .unwrap()
                            .weapon_data.as_mut()
                            .ammo
                            .unwrap()
                            .shoot();
                    })
                />
            </Show>
            <Show when=move|| !has_ammo.get()>
                <div/>
            </Show>
            <div class="weapon_buttons">
                <Show when=move|| has_ammo.get()>
                    <button on:click=move||></button>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn AllArmorView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
}