use std::vec;
use leptos::prelude::*;

#[derive(Clone, Default, Debug, Eq, PartialEq)] 
pub struct ShopModalData {
    visible: bool, 
    pub title: String, 
    pub description: String
}

impl ShopModalData {
    pub fn show(self: &mut Self) {
        self.visible = true;
    }

    pub fn hide(self: &mut Self) {
        self.visible = false;
        self.title = String::new();
        self.description = String::new();
    }
}

#[component]
pub fn ShopModalView(data: RwSignal<ShopModalData>) -> impl IntoView {
    view! {
        <Show when=move||data.get().visible>
            <ShopContent data=data.clone()/>
        </Show>
    }
}


#[component]
pub fn ShopContent(data: RwSignal<ShopModalData>) -> impl IntoView {
    let current_tab = RwSignal::new(0);
    let tabs = vec!["Weapons", "Ammo", "Armor", "Cyberware", "Drugs", "Gear", "Hardware", "Programs"];
    view! {
        <div class="modal" on:click=move |_| data.update(|data| data.hide())>
            <div class="modal-content" on:click=move |ev| { ev.stop_propagation();}>
                <div class="tabs-list">
                    {tabs.into_iter().enumerate().map(|(i, tab_name)| {
                        view! {
                            <div class="tab" 
                                on:click=move|_| current_tab.set(i)
                                class:selected_tab=move|| current_tab.get() == i> 
                                    {tab_name}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <h2>{move|| data.get().title}</h2>
                <hr/>
                <p inner_html={move|| data.get().description}/>
            </div>
        </div>
    }
}