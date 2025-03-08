mod utils;

use nu_cmd_lang::{create_default_context, eval_block};
use nu_protocol::{
    ast::Block,
    engine::{EngineState, StateWorkingSet},
    ir::Instruction,
    CompileError, ParseError, PipelineData, Span, Value,
};
use serde::Serialize;
use std::{path::Path, sync::Arc};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
mod cat;
mod ls;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn consolelog(s: &str);
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
enum RunCodeResult {
    Success(Value),
    ParseErrors(Vec<ParseError>),
    CompileErrors(Vec<CompileError>),
}

fn parse<'engine>(
    contents: &str,
    engine_state: &'engine EngineState,
) -> (Arc<Block>, StateWorkingSet<'engine>) {
    let mut working_set = StateWorkingSet::new(engine_state);
    let output = nu_parser::parse(&mut working_set, None, contents.as_bytes(), false);

    (output, working_set)
}

fn get_engine() -> EngineState {
    let mut engine_state = create_default_context();
    engine_state = nu_command::add_shell_command_context(engine_state);
    engine_state = nu_cmd_extra::add_extra_command_context(engine_state);
    let mut working_set = StateWorkingSet::new(&engine_state);
    working_set.add_decl(Box::new(ls::Ls));
    working_set.add_decl(Box::new(cat::Cat));
    engine_state
        .merge_delta(working_set.delta)
        .expect("Failed to merge delte");
    engine_state
}

fn run_nushell_code(code: &str) -> RunCodeResult {
    let mut engine_state = get_engine();

    let (block, working_set) = parse(&code, &engine_state);

    if !working_set.parse_errors.is_empty() {
        return RunCodeResult::ParseErrors(working_set.parse_errors);
    }
    if !working_set.compile_errors.is_empty() {
        return RunCodeResult::CompileErrors(working_set.compile_errors);
    }
    engine_state.merge_delta(working_set.delta).unwrap();

    let value = eval_block(
        block,
        PipelineData::empty(),
        Path::new("/tmp"),
        &engine_state,
    );
    RunCodeResult::Success(value)
}

#[wasm_bindgen]
pub fn run_code(code: String) -> String {
    set_panic_hook();
    let result = run_nushell_code(&code);
    serde_json::to_string(&result).expect("Failed serializing to json!")
}

#[derive(Serialize)]
struct GetCommandDescriptionResult {
    span: Span,
    description: String,
}

#[wasm_bindgen]
pub fn get_commands_descriptions(code: String) -> String {
    let engine_state = get_engine();
    let (block, _) = parse(&code, &engine_state);

    let commands_descriptions: Vec<GetCommandDescriptionResult> =
        block.ir_block.as_ref().map_or(vec![], |ir_block| {
            ir_block
                .instructions
                .iter()
                .zip(&ir_block.spans)
                .filter_map(|(instruction, span)| match instruction {
                    Instruction::Call {
                        decl_id,
                        src_dst: _,
                    } => Some(GetCommandDescriptionResult {
                        span: *span,
                        description: engine_state.get_decl(*decl_id).description().to_string(),
                    }),
                    _ => None,
                })
                .collect()
        });

    serde_json::to_string(&commands_descriptions).unwrap_or_else(|_| "[]".to_string())
}
