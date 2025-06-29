use leptos::prelude::*;

use cp_char_data::journal::Journal;
use crate::{help::get_char_signal_from_ctx, icon_views::{AddIcon, RemoveIcon}};

#[component]
pub fn TabView(selected_tab_index: RwSignal<usize>, tabs_list: Memo<Vec<String>>,editable_tabs: bool, #[prop(optional)] on_add_tab: Option<Callback<String>>, #[prop(optional)] on_remove_tab: Option<Callback<()>>) -> impl IntoView {
    let show_new_tab_input_signal = RwSignal::new(false);
    view! {
        <div class="journal_tabs"> 
            <Show when=move || editable_tabs>
                <RemoveIcon on:click=move|_| {
                    if on_remove_tab.is_some() {
                        on_remove_tab.unwrap().run(());
                    }
                }/>
            </Show>
            <Show
                when=move|| {editable_tabs && !show_new_tab_input_signal.get()}
            >
                <AddIcon on:click=move|_| show_new_tab_input_signal.set(true) />
            </Show>
            <Show
                when=move|| {editable_tabs && show_new_tab_input_signal.get()}
            >
                <input class="new_journal_input" value="" on:change=move|ev| {
                    let value = event_target_value(&ev);
                    if on_add_tab.is_some() {
                        on_add_tab.unwrap().run(value);
                    }
                    show_new_tab_input_signal.set(false);
                }/>
            </Show>
            <For 
                each=move||{0..(tabs_list.get().len())}
                key=move|index| {index.to_string()}
                children=move|index| {
                    let tab_memo = Memo::new(move |_| {
                        tabs_list.read().get(index).expect("tab should exist inside tab list").clone()
                    });
                    view! {
                        <div 
                            class:selected_tab=move||selected_tab_index.get() == index 
                            on:click=move|_| selected_tab_index.set(index)>
                                {move||tab_memo()}
                        </div>
                    }
                }
            />
        </div>
    }
}


#[component]
pub fn TextCenterSection() -> impl IntoView {
    let cyberpunk_signal = get_char_signal_from_ctx();
    let journal_index: RwSignal<usize> = RwSignal::new(0);
    let center_journal_memo = Memo::new(move|_| {
        let index = journal_index.get();
        cyberpunk_signal.read().journals.get(index).cloned().expect("journal should exist inside character")
    });
    let journal_tabs_memo = Memo::new(move |_| {
        cyberpunk_signal.read().journals.iter().map(|journal| journal.name.clone()).collect::<Vec<String>>()
    });


    view! {
        <section class="flex_col journal_section">
            <div class="journal_tab_wrapper">
                <TabView 
                    selected_tab_index=journal_index 
                    tabs_list=journal_tabs_memo
                    editable_tabs=true 
                    on_add_tab=Callback::new(move|title: String| {
                        cyberpunk_signal.write().journals.push(
                            Journal{
                                name: title,
                                text: "".to_string()
                            }
                        );
                    })
                    on_remove_tab=Callback::new(move|_| {
                        if journal_index.get() == 0 {
                            return;
                        }
                        cyberpunk_signal.write().journals.remove(journal_index.get());
                        journal_index.update(|val| *val -= 1);
                    })
                />
            </div>
            <textarea 
                class="center_text_area" 
                on:change=move |event| {
                    cyberpunk_signal.update(|c| {
                        let val: String = event_target_value(&event);
                        c.journals.get_mut(journal_index.get()).and_then(|journal|{
                            journal.text = val.clone();
                            Some(val)
                        });
                    })
                }
                prop:value=move || center_journal_memo.get().text.clone()
            />
        </section>
    }
}