mod utils;

use nu_cmd_lang::{create_default_context, eval_block, parse};
use nu_command::Sort;
use nu_parser;
use nu_protocol::{
    engine::{EngineState, StateDelta, StateWorkingSet},
    PipelineData,
};
use std::path::Path;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn consolelog(s: &str);
}

#[wasm_bindgen]
pub fn run_code(code: String) -> String {
    set_panic_hook();
    let mut ctx = EngineState::new();
    let mut s = StateWorkingSet::new(&ctx);
    s.add_decl(Box::new(Sort));
    ctx.merge_delta(s.delta).expect("Failed to merge delte");

    let (block, _) = parse(&code, &ctx);
    let val = eval_block(block, PipelineData::empty(), Path::new("/tmp"), &ctx);
    consolelog("result:");
    consolelog(&val.to_debug_string());
    return serde_json::to_string(&val).expect("Failed serizling to string!");
}
