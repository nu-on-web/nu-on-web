use nu_cmd_lang::{create_default_context, eval_block};
use nu_protocol::{
    ast::{Block, Expr, Expression, FindMapResult, Traverse},
    engine::{Command, EngineState, StateWorkingSet},
    ir::Instruction,
    CompileError, DeclId, ParseError, PipelineData, Span, Value,
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

    fn parse<'engine>(&'engine self, contents: &str) -> (Arc<Block>, StateWorkingSet<'engine>) {
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
        let delta = working_set.render();

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

    pub fn get_commands_descriptions(&self, code: &str) -> Vec<GetCommandDescriptionResult> {
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
    pub fn get_pipeline_element_by_position(
        &self,
        code: &str,
        offset: usize,
    ) -> Option<Expression> {
        let next_span_start = self.engine_state.next_span_start();
        let (block, working_set) = self.parse(code);
        block
            .find_map(&working_set, &|expr| {
                find_pipeline_element_by_position(expr, &working_set, next_span_start + offset)
            })
            .cloned()
    }

    pub fn get_declaration_by_id(&self, decl_id: DeclId) -> &dyn Command {
        self.engine_state.get_decl(decl_id)
    }

    pub fn get_next_span_start(&self) -> usize {
        self.engine_state.next_span_start()
    }

    pub fn fetch_completions(&mut self, code: &str, pos: usize) -> (Option<Span>, Vec<String>) {
        let offset = self.engine_state.next_span_start();
        let (block, working_set) = self.parse(code);

        let pos_to_search = pos + offset;

        let Some(element_expression) = block.find_map(&working_set, &|expr: &Expression| {
            find_pipeline_element_by_position(expr, &working_set, pos_to_search)
        }) else {
            return (None, vec![]);
        };

        if let Expr::ExternalCall(expr, _) = &element_expression.expr {
            let start_offset = element_expression.span.start - offset;
            let Some(prefix) = code.get(start_offset..pos) else {
                return (None, vec![]);
            };

            (
                Some(expr.span),
                self.engine_state
                    .find_commands_by_predicate(|name| name.starts_with(prefix.as_bytes()), true)
                    .into_iter()
                    .filter_map(|(_, bytes, _, _)| String::from_utf8(bytes).ok())
                    .collect(),
            )
        } else {
            (None, vec![])
        }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

// Copied from nu-cli crate (https://github.com/nushell/nushell/blob/8352a09117f9d0f40204ca1fc4b191d800d1cb77/crates/nu-cli/src/completions/completer.rs#L23).
fn find_pipeline_element_by_position<'a>(
    expr: &'a Expression,
    working_set: &'a StateWorkingSet,
    offset: usize,
) -> FindMapResult<&'a Expression> {
    // skip the entire expression if the position is not in it
    if !expr.span.contains(offset) {
        return FindMapResult::Stop;
    }
    let closure =
        |expr: &'a Expression| find_pipeline_element_by_position(expr, working_set, offset);
    match &expr.expr {
        Expr::Call(call) => call
            .arguments
            .iter()
            .find_map(|arg| arg.expr().and_then(|e| e.find_map(working_set, &closure)))
            // if no inner call/external_call found, then this is the inner-most one
            .or(Some(expr))
            .map(FindMapResult::Found)
            .unwrap_or_default(),
        Expr::ExternalCall(head, arguments) => arguments
            .iter()
            .find_map(|arg| arg.expr().find_map(working_set, &closure))
            .or(head.as_ref().find_map(working_set, &closure))
            .or(Some(expr))
            .map(FindMapResult::Found)
            .unwrap_or_default(),
        // complete the operator
        Expr::BinaryOp(lhs, _, rhs) => lhs
            .find_map(working_set, &closure)
            .or(rhs.find_map(working_set, &closure))
            .or(Some(expr))
            .map(FindMapResult::Found)
            .unwrap_or_default(),
        Expr::FullCellPath(fcp) => fcp
            .head
            .find_map(working_set, &closure)
            .or(Some(expr))
            .map(FindMapResult::Found)
            .unwrap_or_default(),
        Expr::Var(_) => FindMapResult::Found(expr),
        Expr::AttributeBlock(ab) => ab
            .attributes
            .iter()
            .map(|attr| &attr.expr)
            .chain(Some(ab.item.as_ref()))
            .find_map(|expr| expr.find_map(working_set, &closure))
            .or(Some(expr))
            .map(FindMapResult::Found)
            .unwrap_or_default(),
        _ => FindMapResult::Continue,
    }
}
