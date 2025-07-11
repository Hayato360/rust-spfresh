mod api;
mod app;
mod models;
mod pages;

use leptos::*;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    mount_to_body(|| {
        view! { <App/> }
    })
}
