use nu_cmd_lang::{create_default_context, eval_block};
use nu_protocol::{
    ast::Block,
    engine::{EngineState, StateWorkingSet},
    ir::Instruction,
    CompileError, ParseError, PipelineData, Span, Value,
};
use serde::Serialize;
use std::{path::Path, sync::Arc};

use crate::commands;

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub enum RunCodeResult {
    Success(Value),
    ParseErrors(Vec<ParseError>),
    CompileErrors(Vec<CompileError>),
}

#[derive(Serialize)]
pub struct GetCommandDescriptionResult {
    span: Span,
    description: String,
}

pub struct Engine {
    engine_state: EngineState,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine_state = create_default_context();
        engine_state = nu_command::add_shell_command_context(engine_state);
        engine_state = nu_cmd_extra::add_extra_command_context(engine_state);
        let mut working_set = StateWorkingSet::new(&engine_state);
        working_set.add_decl(Box::new(commands::Ls));
        working_set.add_decl(Box::new(commands::Cat));
        working_set.add_decl(Box::new(commands::Rm));
        engine_state
            .merge_delta(working_set.delta)
            .expect("Failed to merge delta");

        Self { engine_state }
    }

    fn parse<'engine>(&'engine mut self, contents: &str) -> (Arc<Block>, StateWorkingSet<'engine>) {
        let mut working_set = StateWorkingSet::new(&self.engine_state);
        let output = nu_parser::parse(&mut working_set, None, contents.as_bytes(), false);

        (output, working_set)
    }

    pub fn run_code(&mut self, code: &str) -> RunCodeResult {
        let (block, working_set) = self.parse(code);

        if !working_set.parse_errors.is_empty() {
            return RunCodeResult::ParseErrors(working_set.parse_errors);
        }
        if !working_set.compile_errors.is_empty() {
            return RunCodeResult::CompileErrors(working_set.compile_errors);
        }
        let delta = working_set.delta;
        self.engine_state
            .merge_delta(delta)
            .expect("engine state merge failed");

        let value = eval_block(
            block,
            PipelineData::empty(),
            Path::new("/tmp"),
            &self.engine_state,
        );
        RunCodeResult::Success(value)
    }

    pub fn get_commands_descriptions(&mut self, code: &str) -> Vec<GetCommandDescriptionResult> {
        let (block, _) = self.parse(code);

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
                        description: self
                            .engine_state
                            .get_decl(*decl_id)
                            .description()
                            .to_string(),
                    }),
                    _ => None,
                })
                .collect()
        })
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
