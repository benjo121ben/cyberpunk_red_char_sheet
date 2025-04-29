use leptos::prelude::*;

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct SimpleModalData {
    visible: bool, 
    pub title: String, 
    pub description: String
}

impl SimpleModalData {
    pub fn show(self: &mut Self) {
        self.visible = true;
    }

    pub fn reset(self: &mut Self) {
        self.visible = false;
        self.title = String::new();
        self.description = String::new();
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
                    <p inner_html={move|| data.get().description}/>
                </div>
            </div>
        </Show>
    }
}