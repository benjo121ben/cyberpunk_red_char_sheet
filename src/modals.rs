use cp_char_data::{char::Skill, critical_injury};
use leptos::prelude::*;

use crate::help::get_char_signal_from_ctx;


#[component]
pub fn AddSkillModalView(visible: RwSignal<bool>) -> AnyView {
    let char_signal = get_char_signal_from_ctx();
    let selection_signal = RwSignal::new("placeholder".to_string());
    let name_signal = RwSignal::new("".to_string());
    view! {
        <div class="modal" on:click=move |_| visible.set(false)>
            <div class="small_modal_content" on:click=move |ev| { ev.stop_propagation();}>
                <select 
                    on:change:target=move |ev| {
                        let val = ev.target().value();
                        selection_signal.set(val);
                    }
                >
                    <option 
                        value="placeholder">
                        "Select language or local expert"
                    </option>
                    <option 
                        value="Language">
                        "Language"
                    </option>
                    <option 
                        value="Local Expert">
                        "Local Expert"
                    </option>
                </select>
                <input bind:value=name_signal disabled=move||selection_signal.get().as_str() =="placeholder" type="text"/>
                <button type="button" 
                    disabled=move||{
                        selection_signal.get().as_str() == "placeholder" 
                        || name_signal.get().len() < 2
                    }
                    on:click=move |ev| {
                        ev.stop_propagation();
                        let key = selection_signal.get() + " (" + name_signal.get().as_str() + ")";
                        let lowercase_name = key.to_string().to_lowercase().replace(" ", "_");
                        char_signal.update(|punk|{
                            punk.skills.insert(lowercase_name, Skill { name: key.to_string(), nr: 1, difficult_train: false, stat: "int".to_string() });
                        });
                        selection_signal.set("placeholder".to_string());
                        name_signal.set("".to_string());
                        visible.set(false);
                    }
                >
                    ADD
                </button>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn AddInjuryModal(visible: RwSignal<bool>) -> AnyView {
    let char_signal = get_char_signal_from_ctx();
    let selection_signal = RwSignal::new(-1);
    let head_signal = RwSignal::new(false);
    let error_signal = RwSignal::new("".to_string());

    let present_injuries = Memo::new(move |_| {
        if head_signal.get() {
            char_signal.read().head_crit_injuries.clone()
        }
        else {
            char_signal.read().body_crit_injuries.clone()
        }
    });

    let possible_head_injuries = Memo::new(move |_| {
        let mut ret = vec![];
        for (injur_indx, injury) in critical_injury::HEAD_CRIT_INJURIES.iter().enumerate() {
            if char_signal.read().head_crit_injuries.iter().find(|active_injury_index|**active_injury_index == injur_indx).is_some() {
                continue;
            }
            ret.push((injur_indx, injury));
        }
        ret
    });

    let possible_body_injuries = Memo::new(move |_| {
        let mut ret = vec![];
        for (injur_indx, injury) in critical_injury::BODY_CRIT_INJURIES.iter().enumerate() {
            if char_signal.read().body_crit_injuries.iter().find(|active_injury_index|**active_injury_index == injur_indx).is_some() {
                continue;
            }
            ret.push((injur_indx, injury));
        }
        ret
    });

    let injury_options = Memo::new(move |_| {        
        if head_signal.get() {
            possible_head_injuries()
        }
        else {
            possible_body_injuries()
        }

    });

    let change_head_val = move || {
        head_signal.update(|val| *val= !*val);
        selection_signal.set(-1);
    };

    view! {
        <div class="modal" on:click=move |_| visible.set(false)>
            <div class="small_modal_content" on:click=move |ev| { ev.stop_propagation();}>
                <div class="flex_row justify_center">
                    <input type="checkbox" on:click=move |_| change_head_val() prop:checked=move||head_signal.get()/>
                    <span on:click=move|_| change_head_val()>Head</span>
                </div>
                <span>{move || error_signal.get()}</span>
                <select 
                    on:change:target=move |ev| {
                        let val = ev.target().value().parse::<i32>().unwrap();
                        selection_signal.set(val);
                    }
                >
                    <option 
                        value="-1">
                        "Select critical Injury"
                    </option>
                    <For
                        each=move|| {0..injury_options.get().len()}
                        key=move |index| index.to_string() 
                        children=move|index| {
                            let injury_memo = Memo::new(move |_| {
                                injury_options.read()
                                    .get(index)
                                    .cloned()
                                    .unwrap()
                            });
                            view! {
                                <option 
                                    value=move || injury_memo.get().0>
                                    {move || injury_memo.get().1.name}
                                </option>
                            }
                        }
                    />
                </select>
                <button type="button"
                    disabled=move || {selection_signal.get() == -1}
                    on:click=move |ev| {
                        ev.stop_propagation();
                        let index = selection_signal.get();
                        if index == -1 {
                            return;
                        }

                        let already_present = present_injuries.get().iter().find(|val| **val == index as usize).is_some();
                        if already_present {
                            error_signal.set("the character already has this critical injury.".to_string());
                            return;
                        }

                        char_signal.update(|punk|{
                            if head_signal.get() {
                                punk.head_crit_injuries.push(index as usize);
                            }
                            else {
                                punk.body_crit_injuries.push(index as usize);
                            }
                        });
                        
                        selection_signal.set(-1);
                        head_signal.set(false);
                        error_signal.set("".to_string());
                        visible.set(false);
                    }
                >
                    ADD
                </button>
            </div>
        </div>
    }.into_any()
}