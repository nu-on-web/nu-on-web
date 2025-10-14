use std::panic;

use serde::Serialize;
use tsify::Tsify;

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase", tag = "valueType")]
pub enum Value {
    Bool {
        val: bool,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Int {
        val: i64,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Float {
        val: f64,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    String {
        val: String,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Glob {
        val: String,
        no_expand: bool,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Duration {
        /// The duration in nanoseconds.
        val: i64,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    List {
        vals: Vec<Value>,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Error {
        error: Box<ShellError>,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Binary {
        val: Vec<u8>,
        #[serde(rename = "span")]
        internal_span: Span,
    },
    Nothing {
        #[serde(rename = "span")]
        internal_span: Span,
    },
}

impl From<nu_protocol::Value> for Value {
    fn from(value: nu_protocol::Value) -> Self {
        match value {
            nu_protocol::Value::Bool { val, internal_span } => Value::Bool {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Int { val, internal_span } => Value::Int {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Float { val, internal_span } => Value::Float {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::String { val, internal_span } => Value::String {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Glob {
                val,
                no_expand,
                internal_span,
            } => Value::Glob {
                val,
                no_expand,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Duration { val, internal_span } => Value::Duration {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::List {
                vals,
                internal_span,
            } => Value::List {
                vals: vals.into_iter().map(Value::from).collect(),
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Error {
                error,
                internal_span,
            } => Value::Error {
                error: Box::new((*error).into()),
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Binary { val, internal_span } => Value::Binary {
                val,
                internal_span: internal_span.into(),
            },
            nu_protocol::Value::Nothing { internal_span } => Value::Nothing {
                internal_span: internal_span.into(),
            },
            v => panic!("Unsupported value type: {:?}", v),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase", tag = "errorType")]
pub enum ShellError {
    GenericError {
        error: String,
        msg: String,
        span: Option<Span>,
        help: Option<String>,
        inner: Vec<ShellError>,
    },
}

impl From<nu_protocol::ShellError> for ShellError {
    fn from(error: nu_protocol::ShellError) -> Self {
        match error {
            nu_protocol::ShellError::GenericError {
                error,
                msg,
                span,
                help,
                inner,
            } => ShellError::GenericError {
                error,
                msg,
                span: span.map(|s| s.into()),
                help,
                inner: inner.into_iter().map(|e| e.into()).collect(),
            },
            v => panic!("Unsupported shell error type: {:?}", v),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum RunCodeResult {
    Success(Value),
    Error(ShellError),
    ParseErrors(Vec<ParseError>),
    CompileErrors(Vec<CompileError>),
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ParseError {
    span: Span,
    message: String,
}

impl From<nu_protocol::ParseError> for ParseError {
    fn from(error: nu_protocol::ParseError) -> Self {
        ParseError {
            span: error.span().into(),
            message: error.to_string(),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct CompileError {
    message: String,
}

impl From<nu_protocol::CompileError> for CompileError {
    fn from(error: nu_protocol::CompileError) -> Self {
        CompileError {
            message: error.to_string(),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<nu_protocol::Span> for Span {
    fn from(span: nu_protocol::Span) -> Self {
        Span {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct GetCommandDescriptionResult {
    pub span: Span,
    pub description: String,
}

#[derive(Serialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct FetchCompletionResult {
    pub span: Option<Span>,
    pub completions: Vec<String>,
}

impl From<(Option<nu_protocol::Span>, Vec<String>)> for FetchCompletionResult {
    fn from((span, completions): (Option<nu_protocol::Span>, Vec<String>)) -> Self {
        FetchCompletionResult {
            span: span.map(|s| s.into()),
            completions,
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    pub expr: Expr,
    pub span: Span,
}

impl From<nu_protocol::ast::Expression> for Expression {
    fn from(expr: nu_protocol::ast::Expression) -> Self {
        Expression {
            expr: expr.expr.into(),
            span: expr.span.into(),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Expr {
    Call(Call),
}

impl From<nu_protocol::ast::Expr> for Expr {
    fn from(expr: nu_protocol::ast::Expr) -> Self {
        match expr {
            nu_protocol::ast::Expr::Call(call) => Expr::Call((*call).into()),
            v => panic!("Unsupported expression type: {:?}", v),
        }
    }
}

#[derive(Serialize, Debug, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub decl_id: usize,
    pub head: Span,
}

impl From<nu_protocol::ast::Call> for Call {
    fn from(call: nu_protocol::ast::Call) -> Self {
        Call {
            decl_id: call.decl_id.get(),
            head: call.head.into(),
        }
    }
}
