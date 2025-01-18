mod utils;

use std::sync::Arc;

use nu_protocol::engine::{EngineState, Stack};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn highlight(code: &str) {
    let state = EngineState::new();
    let stack = Stack::new();
    alert(code);
}
