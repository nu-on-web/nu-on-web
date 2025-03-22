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
    ) -> Result<PipelineData, ShellError> {
        let path = call.get_flag::<String>(engine_state, stack, "path")?;
        let path = path.unwrap_or_else(|| ".".to_string());
        let span = input.span().unwrap_or(call.head);
        let metadata = input.metadata();
        Ok(Value::list(
            readdir(&path)
                .map_err(|e| ShellError::GenericError {
                    msg: format!("error: {}", e.to_string()),
                    error: format!("error: {}", e.to_string()),
                    span: Some(call.head),
                    help: None,
                    inner: Vec::new(),
                })?
                .into_iter()
                .map(move |f| {
                    let record = {
                        let mut record = Record::new();
                        let stats = stat(&f).map_err(|e| ShellError::GenericError {
                            msg: format!("error: {}", e.to_string()),
                            error: format!("error: {}", e.to_string()),
                            span: Some(call.head),
                            help: None,
                            inner: Vec::new(),
                        })?;
                        record.insert("name", Value::string(f, span));
                        record.insert("size", Value::filesize(stats.size, span));
                        record.insert(
                            "type",
                            Value::string(if stats.is_directory { "dir" } else { "file" }, span),
                        );
                        record
                    };
                    Ok(Value::record(record, span))
                })
                .collect::<Result<Vec<_>, ShellError>>()?,
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
