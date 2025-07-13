use leptos::prelude::*;
use app::App;

mod app;
mod components;

fn main() {
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    console_error_panic_hook::set_once();

    console_error_panic_hook::set_once(); // enables better panic messages
    mount_to_body(|| view! { <App/> });
}
