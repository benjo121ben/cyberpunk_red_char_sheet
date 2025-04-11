use leptos::prelude::*;
use super::char::Character;

pub fn get_char_signal_from_ctx() -> RwSignal<Character>{
    let char_signal_opt: Option<RwSignal<Character>> = use_context();
    match char_signal_opt {
        Some(char_signal) => char_signal,
        None => panic!("The character should have been provided at this point"),
    }
}