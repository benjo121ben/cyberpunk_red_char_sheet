use leptos::prelude::*;
use std::cmp::{min, max};

use crate::help::get_char_signal_from_ctx;


#[component]
pub fn AmmoView(count: Memo<i32>) -> impl IntoView {
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

    }
}

#[component]
pub fn HealthView() -> impl IntoView {
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
    view! {
        <div class="armor_bar" class:head_armor_bar=move||head class:armor_bar_empty=move|| {get_current_sp.get() <= nr}/>
    }

}

#[component]
pub fn ArmorView(head: bool) -> impl IntoView {
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
        armor.map(|a| a.armor_data.sp).or(Some(0)).unwrap()
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
                let max_sp = armor.armor_data.sp;
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
    }
}
