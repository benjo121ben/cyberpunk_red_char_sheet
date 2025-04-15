use leptos::prelude::*;
use crate::char::Character;
use crate::gear::GearData;

use super::char::Skill;
use super::help::get_char_signal_from_ctx;

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

    let entry_clone = entry.clone();
    let has_penalty = Memo::new(move |_| {
        let penalty = rw_char_signal.read().get_current_armor_penalty();
        penalty != 0 && (entry_clone == "ref" || entry_clone == "dex" || entry_clone == "move")
    });
    view! {
        <div class="stat_entry">
            <div class="stat_header">
                {let name_clone = entry.clone(); move|| name_clone.clone()}
            </div>
            <div
                class:has_penalty=move||has_penalty()>
                {move|| stat_memo()}
            </div>
        </div>
    }.into_any()
}



#[component]
pub fn SkillList() -> impl IntoView {
    let rw_char_signal = get_char_signal_from_ctx();
    let filter_flag_memo = Memo::new(move |_| rw_char_signal.read().has_active_flag("filter_zeros"));
    let group_flag_memo = Memo::new(move |_| rw_char_signal.read().has_active_flag("group_by_stat"));
    let skill_list_memo = Memo::new(move |_| {
        let mut temp_list: Vec<(String, Skill)> = rw_char_signal.with(|c| c.skills.clone().into_iter().collect::<Vec<(String, Skill)>>());

        if filter_flag_memo.get() {
            temp_list = temp_list.into_iter().filter(|(_, skill)| skill.nr != 0).collect::<Vec<(String, Skill)>>();
        }
        
        /*if group_flag_memo.get() {
            temp_list.sort_by(|(_, first_skill), (_, second_skill)| first_skill.cmp_stat_and_name(second_skill));
        }
        else {
            temp_list.sort_by(|(_, first_skill), (_, second_skill)| first_skill.cmp_name(second_skill));
        }*/
        return temp_list
    });

    view! {
        <Show when=move || !group_flag_memo.get()>
            <For
                each=move||{skill_list_memo.get()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
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
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">DEX</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "dex").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">EMP</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "emp").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">INT</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "int").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">REF</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "ref").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">TECH</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "tech").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
            <div class="skill_list_cat_header">WILL</div>
            <For
                each=move||{skill_list_memo.get().into_iter().filter(|(_, skill)| skill.stat.as_str() == "will").collect::<Vec<(String, Skill)>>()}
                key=|(key, _)| key.clone()
                children=move |(key, _)| {
                    view! {
                        <SkillEntry key=key.clone()/>
                    }
                }
            /> 
        </Show>
    }

}

#[component]
fn SkillEntry(key: String) -> impl IntoView {
    let char_signal = get_char_signal_from_ctx();
    let key_clone = key.clone(); 
    let skill_memo = Memo::new(move |_| char_signal.with(|c| c.skills.get(&key).expect("expect skill to exist in its own list").clone()));
    let get_skill_value = move || {
        let skill = skill_memo.get();
        char_signal.with(|char| char.get_stat(&skill.stat.clone())) + skill.nr
    };

    let update_skill = move|val: i32| {
        char_signal.update(|c| {
            c.skills.get_mut(&key_clone).and_then(|skill| {
                skill.nr += val;
                Some(skill)
            });
        })
    };
    

    //todo benji add armor penalty visual, stat is already adjusted
    let has_penalty = Memo::new(move |_| {
        let stat = skill_memo.get().stat;
        let penalty = char_signal.read().get_current_armor_penalty();
        penalty != 0 && (stat == "ref" || stat == "dex" || stat == "move")
    });

    let update_skill_clone = update_skill.clone();

    view! {
        <div class="skill_entry_name" 
            title={move || skill_memo.read().stat.to_uppercase().clone()}
            class:has_penalty=move|| has_penalty()>
                {move || skill_memo.read().name.clone()}
        </div>
        <div class="skill_entry_value" 
            class:has_penalty=move|| has_penalty()
            on:click=move|_| update_skill(1) 
            on:contextmenu=move|_| update_skill_clone(-1)>
                {get_skill_value}
        </div>
    }
}
