use cp_char_data::char::Skill;
use leptos::prelude::*;

use crate::help::get_char_signal_from_ctx;


#[component]
pub fn AddSkillModalView(visible: RwSignal<bool>) -> AnyView {
    let char_signal = get_char_signal_from_ctx();
    let selection_signal = RwSignal::new("placeholder".to_string());
    let name_signal = RwSignal::new("".to_string());
    view! {
        <div class="modal" on:click=move |_| visible.set(false)>
            <div class="add_skill_content" on:click=move |ev| { ev.stop_propagation();}>
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