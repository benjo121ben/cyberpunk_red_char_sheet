use indexmap::IndexMap;
use leptos::prelude::*;
use cp_char_data::char::Character;

pub fn get_char_signal_from_ctx() -> RwSignal<Character>{
    let char_signal_opt: Option<RwSignal<Character>> = use_context();
    match char_signal_opt {
        Some(char_signal) => char_signal,
        None => panic!("The character should have been provided at this point"),
    }
}

pub fn reduce_or_remove_items_in_map (item_map: &mut IndexMap<String, i32>, key: &str) {
    match item_map.get_mut(key) {
        Some(value) => {
            if *value <= 1 {
                item_map.shift_remove(key);
            }
            else {
                *value -= 1;
            }
        },
        None => todo!(),
    }
}