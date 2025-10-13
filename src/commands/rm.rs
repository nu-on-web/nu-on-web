use nu_engine::CallExt;
use nu_protocol::{
    engine::Command, Example, PipelineData, ShellError, Signature, SyntaxShape, Type,
};

use crate::zenfs::unlink;

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
        let path: String = call.req(engine_state, stack, 0)?;
        unlink(&path).map_err(|e| ShellError::GenericError {
            msg: e.to_string().into(),
            error: e.to_string().into(),
            span: Some(call.head),
            help: None,
            inner: Vec::new(),
        })?;

        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example<'static>> {
        vec![
            Example {
                description: "Remove a file",
                example: "rm myfile.txt",
                result: None,
            },
            Example {
                description: "Remove a file in a subdirectory",
                example: "rm subdir/myfile.txt",
                result: None,
            },
        ]
    }
}
