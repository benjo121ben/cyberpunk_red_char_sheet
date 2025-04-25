use leptos::prelude::*;

use crate::help::get_char_signal_from_ctx;

#[component]
pub fn TextCenterSection() -> impl IntoView {
    let cyberpunk_signal = get_char_signal_from_ctx();
    let journal_index: RwSignal<usize> = RwSignal::new(1);
    let center_journal_memo = Memo::new(move|_| {
        let index = journal_index.get();
        cyberpunk_signal.read().journals.get(index).cloned().expect("journal should exist inside character")
    });
    view! {
        <section class="flex_col">
            <div class="journal_tabs"> 
                <For 
                    each=move||{1..(cyberpunk_signal.read().journals.len())}
                    key=move|index| {index.to_string()}
                    children=move|index| {
                        let header_memo = Memo::new(move |_| {
                            cyberpunk_signal.read().journals.get(index).expect("journal should exist inside character").name.clone()
                        });
                        view! {
                            <div 
                                class:selected_tab=move||journal_index.get() == index 
                                on:click=move|_| journal_index.set(index)>
                                    {move||header_memo()}
                            </div>
                        }
                    }
                />
            </div>
            <textarea 
                class="center-text-area" 
                class:first-tab-selected=move||journal_index() == 0
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