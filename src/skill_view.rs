use leptos::{ev::MouseEvent, prelude::*};
use std::cmp::{max, min};

use cp_char_data::char::Skill;
use super::help::get_char_signal_from_ctx;

const IP_SPENDING_TABLE: &[i32] = &[20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

#[component]
pub fn StatsView() -> impl IntoView {
    let all_stats: Vec<String> = vec!["int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body", "emp"].into_iter().map(|val|val.to_string()).collect();
    view! {
        <div class="stat_view">
            <For each= move || all_stats.clone()
                key=move|val| val.clone()
                children=move |entry| {
                    view!{<SingleStatEntryView entry/>}
                }
            />
        </div>
    }
}


#[component]
fn SingleStatEntryView(entry: String) -> impl IntoView {
    let rw_char_signal = get_char_signal_from_ctx();

    let entry_clone = entry.clone();
    let stat_memo = Memo::new(move |_| {
        rw_char_signal.read().get_stat(&entry_clone)
    });

    let has_penalty = move || {
        stat_memo.get().1
    };

    let entry_clone = entry.clone();
    let decrease_luck = move || {
        if entry_clone != "luck" {
            return
        }
        let current_luck = rw_char_signal.read().stats.luck_current;
        rw_char_signal.write().stats.luck_current = max(current_luck - 1, 0);
    };

    let entry_clone = entry.clone();
    let reset_luck = move || {
        if entry_clone != "luck" {
            return
        }
        let luck_max = rw_char_signal.read().stats.luck;
        rw_char_signal.write().stats.luck_current = luck_max;
    };

    view! {
        <div class="stat_entry"
            on:click=move |ev| {ev.stop_propagation(); ev.prevent_default(); decrease_luck();}
            on:contextmenu=move |ev| {ev.stop_propagation(); ev.prevent_default(); reset_luck();}
        >
            <div class="stat_header">
                {let name_clone = entry.clone(); move|| name_clone.clone()}
            </div>
            <div
                class:has_penalty=move||has_penalty()>
                {move || if entry == "luck" {format!("{} / ", rw_char_signal.read().stats.luck_current)} else {"".to_string()}} {move|| stat_memo().0}
            </div>
        </div>
    }.into_any()
}



#[component]
pub fn SkillList(unlocked_signal: RwSignal<bool>) -> impl IntoView {
    let rw_char_signal = get_char_signal_from_ctx();
    let filter_flag_memo = Memo::new(move |_| rw_char_signal.read().has_active_flag("filter_zeros"));
    let group_flag_memo = Memo::new(move |_| rw_char_signal.read().has_active_flag("group_by_stat"));
    let skill_list_memo = Memo::new(move |_| {
        let mut temp_list: Vec<(String, Skill)> = rw_char_signal
            .with(|c| c.skills.clone().into_iter().collect::<Vec<(String, Skill)>>());

        if filter_flag_memo.get() {
            temp_list = temp_list.into_iter().filter(|(_, skill)| skill.nr != 0).collect::<Vec<(String, Skill)>>();
        }
        temp_list.sort_by(|(_, first_skill), (_, second_skill)| first_skill.cmp_name(second_skill));
        return temp_list;
    });

    view! {
        <Show when=move || !group_flag_memo.get()>
            <For
                each=move||{skill_list_memo.get()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
        </Show>
        <Show when=move || group_flag_memo.get()>
            <div class="skill_list_cat_header">COOL</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "cool").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">DEX</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "dex").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">EMP</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "emp").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">INT</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "int").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">REF</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "ref").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">TECH</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "tech").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">WILL</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "will").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry unlocked_signal key=key.clone()/>
                    }
                }
            /> 
        </Show>
    }

}

#[component]
fn SkillEntry(unlocked_signal: RwSignal<bool>, key: String) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let (key_read_signal, _) = signal(key.clone());
    let skill_data_memo = Memo::new(move |_| char_signal.with(|c| c.get_full_skill_data(&key)));

    let skill_value_visual = move || {
        let (skill, bonus, _) = skill_data_memo.get();
        if unlocked_signal.get() {
            skill.nr as i32
        }
        else {
            bonus
        }
    };

    let is_language_or_local_expert = move || {
        let (skill, _, _) = skill_data_memo.get();
        skill.name.contains("Language") || skill.name.contains("Local Expert")
    };

    let update_skill = move|val: i32| {
        if !unlocked_signal.get() {
            return;
        }
        char_signal.update(|c| {
            c.skills.get_mut(&key_read_signal()).and_then(|skill| {
                skill.nr = max(min(skill.nr as i32 + val, 10), 0) as usize;
                Some(skill)
            });
        })
    };

    //todo benji add armor penalty visual, stat is already adjusted
    let has_penalty = move || {
        let (_, _, penalty) = skill_data_memo.get();
        penalty
    };

    let update_skill_clone = update_skill.clone();

    let get_tooltip = move || {
        let (skill, _, _) = skill_data_memo.get();
        let value: usize = skill.nr as usize;
        if unlocked_signal.get() && value < 10 {
            let diff = if skill.difficult_train {2} else {1};
                
            let cost = *IP_SPENDING_TABLE.get(value).expect(&format!("expecting a value to exist for {value}")) * diff;
            
            cost.to_string()
        }
        else {
            skill.stat.to_uppercase()
        }
    };

    view! {
        <div class="skill_entry"
            class:unlocked=move||unlocked_signal.get()
            on:click=move|ev| {ev.stop_propagation(); ev.prevent_default(); update_skill(1); } 
            on:contextmenu=move|ev| {ev.stop_propagation(); ev.prevent_default();  update_skill_clone(-1); }
            title=get_tooltip
        >
            <div class="skill_entry_name" 
                class:has_penalty=move|| has_penalty()>
                    <Show when=move|| unlocked_signal.get() && is_language_or_local_expert()>
                        <button 
                            on:click=move|ev: MouseEvent| {
                                ev.stop_propagation();
                                char_signal.update(|punk| {
                                    punk.skills.shift_remove(&key_read_signal());
                                });
                            }
                        >
                            X
                        </button>
                    </Show>
                    {move || {
                        skill_data_memo.read().0.name.clone() + 
                        if unlocked_signal.get() {
                            skill_data_memo.read().0.difficult_train.then(||" (x2)").or(Some("")).unwrap()
                        }
                        else {
                            ""
                        }

                    }}
            </div>
            <div class="skill_entry_value"
                class:has_penalty=move|| has_penalty()
            >
                {skill_value_visual}
            </div>
        </div>
    }
}
