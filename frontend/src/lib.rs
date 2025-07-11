pub mod api;
pub mod app;
pub mod models;
pub mod pages;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub use app::App;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    mount_to_body(|| {
        view! { <App/> }
    });
}
