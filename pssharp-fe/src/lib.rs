use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()));
}

macro_rules! console_err {
    ($($t:tt)*) => (web_sys::console::error_1(&format_args!($($t)*).to_string().into()));
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("main");
}
