pub mod journal;
pub mod help;
pub mod skill_view;
pub mod resource_views;
pub mod icon_views;
pub mod gear_views;
pub mod shop_modal_view;
pub mod app;
pub mod gear;
pub mod char;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
