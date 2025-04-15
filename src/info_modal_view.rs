use std::vec;

use leptos::prelude::*;

use crate::views::stats_views::TraitView;

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct SimpleModalData {
    visible: bool, 
    pub title: String, 
    pub description: String,
    pub traits: Vec<String>
}

impl SimpleModalData {
    pub fn show(self: &mut Self) {
        self.visible = true;
    }

    pub fn reset(self: &mut Self) {
        self.visible = false;
        self.title = String::new();
        self.description = String::new();
        self.traits = vec![];
    }
    
}

#[component]
pub fn SimpleModalView(data: RwSignal<SimpleModalData>) -> impl IntoView {
    view! {
        <Show when=move||data.get().visible>
            <div class="modal" on:click=move |_| data.update(|data| data.reset())>
                <div class="modal_content" on:click=move |_| {}>
                    <h2>{move|| data.get().title}</h2>
                    <hr/>
                    <Show when=move||{data.get().traits.len() > 0}>
                        {move || {
                            let traits = data.get().traits;
                            view!{
                                <TraitView trait_names=traits/>
                            }
                        }}
                    </Show>
                    <p inner_html={move|| data.get().description}/>
                </div>
            </div>
        </Show>
    }
}