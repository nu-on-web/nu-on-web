use std::any::type_name_of_val;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
pub fn olog(s: String) {
    log(s.as_str())
}

use js_sys::Error;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn failed_to_serialize_error<T>(val: &T) -> Error
where
    T: ?Sized,
{
    Error::new(format!("Failed to serialize {}", type_name_of_val(val)).as_str())
}
