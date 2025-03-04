use nu_engine::command_prelude::*;

use serde::Deserialize;

use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct Stats {
    pub is_directory: bool,
    pub size: i64,
}

#[wasm_bindgen]
extern "C" {
    fn readdir(path: &str) -> Vec<String>;
    fn stat(path: &str) -> JsValue;
}

#[derive(Clone)]
pub struct Ls;

impl Command for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> Signature {
        Signature::build("ls").optional(
            "path",
            SyntaxShape::GlobPattern,
            "a path to get the directory contents from",
        )
    }

    fn description(&self) -> &str {
        "View the contents of the current or given path."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, nu_protocol::ShellError> {
        let path = call.get_flag::<String>(engine_state, stack, "path")?;
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();
        Ok(Value::list(
            readdir(&path.unwrap_or(String::from(".")))
                .into_iter()
                .map(move |f| {
                    let record = {
                        let mut record = Record::new();
                        let v = stat(&f);
                        let stats: Stats = from_value(v).expect("Failed to get stats");
                        record.insert("name", Value::string(f, span));
                        record.insert("size", Value::filesize(stats.size, span));
                        record.insert(
                            "type",
                            Value::string(if stats.is_directory { "dir" } else { "file" }, span),
                        );
                        record
                    };
                    Value::record(record, span)
                })
                .collect(),
            span,
        )
        .into_pipeline_data_with_metadata(metadata))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "List all files in the current directory",
                example: "ls",
                result: None,
            },
            Example {
                description: "List all files in a subdirectory",
                example: "ls subdir",
                result: None,
            },
            Example {
                description: "List all rust files",
                example: "ls *.rs",
                result: None,
            },
        ]
    }
}
