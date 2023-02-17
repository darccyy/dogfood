use leptos::*;

use dogfood::*;

fn main() {
    // Setup console logging
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // Mount app
    mount_to_body(|cx| view! { cx,  <App/> })
}
