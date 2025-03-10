use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Stats {
    pub is_directory: bool,
    pub size: u32,
}

#[wasm_bindgen(module = "@zenfs/core")]
extern "C" {
    #[wasm_bindgen(js_namespace = fs, js_name = readFileSync)]
    pub fn readfile(path: &str) -> String;
    #[wasm_bindgen(js_namespace = fs, js_name = readdirSync)]
    pub fn readdir(path: &str) -> Vec<String>;
    #[wasm_bindgen(js_namespace = fs, js_name = statSync)]
    fn stat_js(path: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = fs, js_name = unlinkSync)]
    pub fn unlink(path: &str);
}

pub fn stat(path: &str) -> Stats {
    let value = stat_js(path);
    if !value.is_object() {
        panic!("Failed to get stats");
    }

    let is_directory =
        Reflect::get(&value, &JsValue::from_str("isDirectory")).expect("Failed to get isDirectory");
    if !is_directory.is_function() {
        panic!("Failed to get isDirectory");
    }
    let is_directory = is_directory
        .dyn_into::<js_sys::Function>()
        .expect("Failed to get isDirectory");
    let is_directory = is_directory
        .call0(&value)
        .expect("Failed to call isDirectory")
        .as_bool()
        .expect("Failed to get isDirectory");

    let size = Reflect::get(&value, &JsValue::from_str("size"))
        .expect("Failed to get size")
        .as_f64()
        .expect("Failed to get size") as u32;

    Stats { is_directory, size }
}
