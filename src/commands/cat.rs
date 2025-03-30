use js_sys::{Object, Reflect};
use nu_engine::CallExt;
use nu_protocol::{
    engine::Command, IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Type,
    Value,
};
use wasm_bindgen::JsValue;

use crate::zenfs::readfile;

#[derive(Clone)]
pub struct Cat;

impl Command for Cat {
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
        "cat file"
    }

    fn run(
        &self,
        engine_state: &nu_protocol::engine::EngineState,
        stack: &mut nu_protocol::engine::Stack,
        call: &nu_protocol::engine::Call,
        input: nu_protocol::PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let path: String = call.req(engine_state, stack, 0)?;
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();

        let options = Object::new();
        let _ = Reflect::set(
            &options,
            &JsValue::from_str("encoding"),
            &JsValue::from_str("utf-8"),
        )
        .expect("Failed to set property");

        Ok(Value::string(
            readfile(&path, options).map_err(|e| ShellError::GenericError {
                msg: format!("error: {}", e.to_string()),
                error: format!("error: {}", e.to_string()),
                span: Some(call.head),
                help: None,
                inner: Vec::new(),
            })?,
            span,
        )
        .into_pipeline_data_with_metadata(metadata))
    }
}
