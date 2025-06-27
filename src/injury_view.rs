use leptos::prelude::*;

use crate::help::get_char_signal_from_ctx;
use cp_char_data::critical_injury;

#[component]
pub fn InjuryView() -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    view! {
        <div class="flex_row justify_center flex_wrap">
            <For
                each=move || {0..char_signal.read().head_crit_injuries.len()}
                key=move |indx| indx.to_string()
                children=move |indx| {
                    let injury = Memo::new(move |_| {
                        let injury_index = char_signal.read().head_crit_injuries.get(indx).cloned().unwrap();
                        critical_injury::HEAD_CRIT_INJURIES.get(injury_index as usize).expect("head_crit injury should exist")
                    });
                    view!{
                        <span
                            on:contextmenu=move|ev|{ev.prevent_default(); ev.stop_propagation(); char_signal.update(|punk| {punk.head_crit_injuries.remove(indx);});}
                            class="nowrap"
                        >
                            {move|| injury.get().name}
                        </span>
                    }
                }
            />
        </div>
        <div class="injury_div">
            <For
                each=move || {0..char_signal.read().body_crit_injuries.len()}
                key=move |indx| indx.to_string()
                children=move |indx| {
                    let injury = Memo::new(move |_| {
                        let injury_index = char_signal.read().body_crit_injuries.get(indx).cloned().unwrap();
                        critical_injury::BODY_CRIT_INJURIES.get(injury_index as usize).expect("body_crit injury should exist")
                    });
                    view!{
                        <span 
                            on:contextmenu=move|ev|{ev.prevent_default(); ev.stop_propagation(); char_signal.update(|punk| {punk.body_crit_injuries.remove(indx);});}
                            class="nowrap"
                        >
                            {move|| injury.get().name}
                        </span>
                    }
                }
            />
        </div>
    }
}