use std::path;

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
            SyntaxShape::String,
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
        let path = call.opt::<String>(engine_state, stack, 0)?;
        let path = path.unwrap_or_else(|| ".".to_string());
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();
        let items = match readdir(&path) {
            Ok(v) => v,
            Err(e) => {
                return Ok(Value::Error {
                    error: Box::new(ShellError::GenericError {
                        msg: format!("error: {}", e.to_string()),
                        error: format!("error: {}", e.to_string()),
                        span: Some(call.head),
                        help: None,
                        inner: Vec::new(),
                    }),
                    internal_span: call.head,
                }
                .into_pipeline_data_with_metadata(metadata));
            }
        };
        Ok(Value::list(
            items
                .into_iter()
                .map(|f| path::Path::new(&path).join(f))
                .map(|path| {
                    path.strip_prefix("./")
                        .unwrap_or(&path)
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .map(move |f| {
                    let record = {
                        let mut record = Record::new();
                        record.insert("name", Value::string(&f, span));

                        let stats = stat(&f);
                        match stats {
                            Ok(stats) => {
                                record.insert("size", Value::filesize(stats.size, span));
                                record.insert(
                                    "type",
                                    Value::string(
                                        if stats.is_directory { "dir" } else { "file" },
                                        span,
                                    ),
                                );
                                record
                            }
                            Err(_) => {
                                record.insert("size", Value::filesize(0, span));
                                record.insert("type", Value::string("?", span));
                                record
                            }
                        }
                    };
                    Value::record(record, span)
                })
                .collect(),
            span,
        )
        .into_pipeline_data_with_metadata(metadata))
    }

    fn examples(&self) -> Vec<Example<'static>> {
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
