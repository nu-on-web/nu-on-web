mod utils;

mod commands;
mod engine;
mod zenfs;

use std::sync::{Mutex, OnceLock};

use engine::Engine;
use js_sys::Error;
use utils::set_panic_hook;
use wasm_bindgen::{prelude::*, JsValue};

static ENGINE: OnceLock<Mutex<Engine>> = OnceLock::new();

fn get_engine() -> &'static Mutex<Engine> {
    ENGINE.get_or_init(|| Mutex::new(Engine::new()))
}

#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn run_code(code: &str) -> Result<JsValue, Error> {
    let mut engine = get_engine().lock().expect("Failed to lock engine");
    serde_wasm_bindgen::to_value(&engine.run_code(code))
        .map_err(|_| Error::new("Failed to serialize RunCodeResult"))
}

#[wasm_bindgen]
pub fn get_commands_descriptions(code: &str) -> Result<JsValue, Error> {
    let mut engine = get_engine().lock().expect("Failed to lock engine");
    serde_wasm_bindgen::to_value(&engine.get_commands_descriptions(code))
        .map_err(|_| Error::new("Failed to serialize Vec<GetCommandDescriptionResult>"))
}
