use js_sys::{Error, Object, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@zenfs/core")]
extern "C" {
    #[wasm_bindgen(js_namespace = fs, js_name = readFileSync, catch)]
    pub fn readfile(path: &str, options: Object) -> Result<String, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = readdirSync, catch)]
    pub fn readdir(path: &str) -> Result<Vec<String>, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = statSync, catch)]
    pub fn stat(path: &str) -> Result<Stats, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = unlinkSync, catch)]
    pub fn unlink(path: &str) -> Result<(), Error>;
}

#[wasm_bindgen]
extern "C" {
    pub type Stats;

    #[wasm_bindgen(structural, method, js_name = isDirectory)]
    pub fn is_directory(this: &Stats) -> bool;
}

impl Stats {
    pub fn size(&self) -> u32 {
        let v = Reflect::get(self.as_ref(), &JsValue::from_str("size"))
            .expect("Failed to get 'size' property from Stats object");
        v.as_f64().expect("Stats 'size' property should be a numeric value") as u32
    }
}
