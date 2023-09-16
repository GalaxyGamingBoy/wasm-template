mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();

    web_sys::console::log_1(&"Hello, WASM!".into());
}
