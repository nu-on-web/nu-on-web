use nu_engine::CallExt;
use nu_protocol::{engine::Command, PipelineData, ShellError, Signature, SyntaxShape, Type};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn unlink(path: String);
}

#[derive(Clone)]
pub struct Rm;

impl Command for Rm {
    fn name(&self) -> &str {
        "rm"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Nothing, Type::Nothing)
            .required(
                "path",
                SyntaxShape::String,
                "the path to the file to remove",
            )
    }

    fn description(&self) -> &str {
        "remove file"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        _input: nu_protocol::PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let path = call.req(engine_state, stack, 0)?;
        unlink(path);
        Ok(PipelineData::empty())
    }
}
