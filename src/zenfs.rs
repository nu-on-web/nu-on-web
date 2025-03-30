use js_sys::{Error, Object, Reflect};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Stats {
    pub is_directory: bool,
    pub size: u32,
}

#[wasm_bindgen(module = "@zenfs/core")]
extern "C" {
    #[wasm_bindgen(js_namespace = fs, js_name = readFileSync, catch)]
    pub fn readfile(path: &str, options: Object) -> Result<String, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = readdirSync, catch)]
    pub fn readdir(path: &str) -> Result<Vec<String>, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = statSync, catch)]
    fn stat_js(path: &str) -> Result<JsValue, Error>;
    #[wasm_bindgen(js_namespace = fs, js_name = unlinkSync, catch)]
    pub fn unlink(path: &str) -> Result<(), Error>;
}

pub fn stat(path: &str) -> Result<Stats, Error> {
    let value = stat_js(path)?;
    if !value.is_object() {
        return Err(Error::new("Failed to get stats"));
    }

    let is_directory = Reflect::get(&value, &JsValue::from_str("isDirectory"))
        .map_err(|_| Error::new("Failed to get isDirectory"))?;
    if !is_directory.is_function() {
        return Err(Error::new("Failed to get isDirectory"));
    }
    let is_directory = is_directory
        .dyn_into::<js_sys::Function>()
        .map_err(|_| Error::new("Failed to get isDirectory"))?;
    let is_directory = is_directory
        .call0(&value)
        .map_err(|_| Error::new("Failed to call isDirectory"))?
        .as_bool()
        .ok_or_else(|| Error::new("Failed to get isDirectory"))?;

    let size = Reflect::get(&value, &JsValue::from_str("size"))
        .map_err(|_| Error::new("Failed to get size"))?
        .as_f64()
        .ok_or_else(|| Error::new("Failed to get size"))? as u32;

    Ok(Stats { is_directory, size })
}
