use nu_engine::CallExt;
use nu_protocol::{
    engine::Command, IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Type,
    Value,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn readfile(path: String) -> String;
}

#[derive(Clone)]
pub struct Open;

impl Command for Open {
    fn name(&self) -> &str {
        "cat"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::String)
            .required(
                "path",
                SyntaxShape::String,
                "a path to get the content from",
            )
    }

    fn description(&self) -> &str {
        "open file"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let path = call.req(engine_state, stack, 0)?;
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();
        Ok(Value::string(readfile(path), span).into_pipeline_data_with_metadata(metadata))
    }
}
