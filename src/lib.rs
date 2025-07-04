pub mod help;
pub mod skill_view;
pub mod resource_views;
pub mod icon_views;
pub mod gear_views;
pub mod netrun_view;
pub mod text_views;
pub mod info_modal_view;
pub mod shop_modal_view;
pub mod injury_view;
pub mod modals;
pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
