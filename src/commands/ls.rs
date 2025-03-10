use nu_engine::command_prelude::*;

use crate::zenfs::{readdir, stat};

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
        let path = path.unwrap_or_else(|| ".".to_string());
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();
        Ok(Value::list(
            readdir(&path)
                .into_iter()
                .map(move |f| {
                    let record = {
                        let mut record = Record::new();
                        let stats = stat(&f);
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
