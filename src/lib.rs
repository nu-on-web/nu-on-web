mod utils;

mod commands;
mod engine;
mod types;
mod zenfs;

use std::sync::{Mutex, OnceLock};

use engine::Engine;
use js_sys::Error;
use nu_protocol::DeclId;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use crate::types::{
    Expression, FetchCompletionResult, GetCommandDescriptionResult, RunCodeResult,
};

static ENGINE: OnceLock<Mutex<Engine>> = OnceLock::new();

fn get_engine() -> &'static Mutex<Engine> {
    ENGINE.get_or_init(|| Mutex::new(Engine::new()))
}

#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();
}

#[wasm_bindgen(js_name = "runCode")]
pub fn run_code(code: &str) -> RunCodeResult {
    let mut engine = get_engine().lock().expect("Failed to lock engine");
    engine.run_code(code)
}

#[wasm_bindgen(js_name = "getCommandsDescriptions")]
pub fn get_commands_descriptions(code: &str) -> Vec<GetCommandDescriptionResult> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    engine.get_commands_descriptions(code)
}

#[wasm_bindgen(js_name = "findPipelineElementByOffset")]
pub fn find_pipeline_element_by_offset(code: &str, offset: usize) -> Option<Expression> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    engine
        .get_pipeline_element_by_offset(code, offset)
        .map(|v| v.into())
}

#[wasm_bindgen(js_name = "getDeclarationNameFromId")]
pub fn get_declaration_name_from_id(decl_id: usize) -> String {
    let engine = get_engine().lock().expect("Failed to lock engine");
    let cmd = engine.get_declaration_by_id(DeclId::new(decl_id));
    cmd.name().into()
}

#[wasm_bindgen(js_name = "getNextSpanStart")]
pub fn get_next_span_start() -> Result<usize, Error> {
    let engine = get_engine().lock().expect("Failed to lock engine");
    Ok(engine.get_next_span_start())
}

#[wasm_bindgen(js_name = "fetchCompletions")]
pub fn fetch_completions(code: &str, pos: usize) -> FetchCompletionResult {
    let mut engine = get_engine().lock().expect("Failed to lock engine");

    engine.fetch_completions(code, pos).into()
}
