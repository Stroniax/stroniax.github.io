use wasm_bindgen::prelude::*;

mod dark_mode;
mod settings;
mod tooltip;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()));
}

macro_rules! console_err {
    ($($t:tt)*) => (web_sys::console::error_1(&format_args!($($t)*).to_string().into()));
}

pub(crate) use console_err;
pub(crate) use console_log;
use web_sys::{window, HtmlElement, MouseEvent};

#[wasm_bindgen(start)]
pub fn main() {
    let window = window().expect("Cannot access window!");

    // watch and forget - this leaks memory to leave the closures running
    // since we do not unsubscribe at any point in time
    dark_mode::watch_storage(&window).forget();

    let on_window_load = Closure::<dyn Fn()>::new(|| {
        settings::init_settings();

        dark_mode::set_styles_from_preference();

        tooltip::init_tooltip();
    });

    window
        .add_event_listener_with_callback("load", on_window_load.as_ref().unchecked_ref())
        .expect("Could not subscribe to window load");

    on_window_load.forget();

    dark_mode::watch_storage(&window).forget();
}
