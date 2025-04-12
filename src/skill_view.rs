use leptos::prelude::*;
use super::char::Skill;
use super::help::get_char_signal_from_ctx;

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

    let update_skill_clone = update_skill.clone();

    view! {
        <div class="skill_entry_name" title={move || skill_memo.read().stat.to_uppercase().clone()}>
            {move || skill_memo.read().name.clone()}
        </div>
        //<div class="skill_entry_stat">{move || skill_memo.read().stat.to_uppercase().clone()}</div>
        <div class="skill_entry_value" 
            on:click=move|_| update_skill(1) 
            on:contextmenu=move|_| update_skill_clone(-1)>
                {get_skill_value}
        </div>
    }
}
