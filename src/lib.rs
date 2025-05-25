mod utils;

mod commands;
mod engine;
mod zenfs;

use std::sync::{Mutex, OnceLock};

use engine::Engine;
use js_sys::Error;
use nu_protocol::DeclId;
use utils::{failed_to_serialize_error, set_panic_hook};
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
    let run_code_result = engine.run_code(code);
    serde_wasm_bindgen::to_value(&run_code_result)
        .map_err(|_| failed_to_serialize_error(&run_code_result))
}

#[wasm_bindgen]
pub fn get_commands_descriptions(code: &str) -> Result<JsValue, Error> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    let commands_description = engine.get_commands_descriptions(code);
    serde_wasm_bindgen::to_value(&commands_description)
        .map_err(|_| failed_to_serialize_error(&commands_description))
}

#[wasm_bindgen]
pub fn find_pipeline_element_by_position(code: &str, pos: usize) -> Result<JsValue, Error> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    let elm = engine.get_pipeline_element_by_position(code, pos);
    serde_wasm_bindgen::to_value(&elm).map_err(|_| failed_to_serialize_error(&elm))
}

#[wasm_bindgen]
pub fn get_declaration(decl_id: usize) -> Result<JsValue, Error> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    let cmd = engine.get_declaration_by_id(DeclId::new(decl_id));
    Ok(cmd.name().into())
}

#[wasm_bindgen]
pub fn get_next_span_start() -> Result<usize, Error> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    Ok(engine.get_next_span_start())
}
