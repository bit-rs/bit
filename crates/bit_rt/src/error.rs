/// Imports
use crate::rt::value::{ChanValue, Value};
use bit_ast::atom::{BinaryOp, UnaryOp};
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Unsafe `Send` + `Sync` implementations.
unsafe impl Send for Value {}
unsafe impl Sync for Value {}

/// Runtime error
#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    /// Undefined variable
    #[error("variable `{name}` is not defined")]
    #[diagnostic(code(rt::undefined_variable))]
    UndefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable access here...")]
        span: SourceSpan,
    },
    /// Undefined field
    #[error("field `{name}` is not defined")]
    #[diagnostic(code(rt::undefined_field))]
    UndefinedField {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("field access here...")]
        span: SourceSpan,
    },
    /// Variable is already defined
    #[error("variable `{name}` is already defined")]
    #[diagnostic(code(rt::already_defined_variable))]
    AlreadyDefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("redeclaration attempt here...")]
        span: SourceSpan,
    },
    /// Invalid binary op
    #[error("couldn't use `{op}` with `{a}` and `{b}`")]
    #[diagnostic(code(rt::invalid_binary_op))]
    InvalidBinaryOp {
        op: BinaryOp,
        a: Value,
        b: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Invalid unary op
    #[error("couldn't use `{op}` with `{value}`")]
    #[diagnostic(code(rt::invalid_unary_op))]
    InvalidUnaryOp {
        op: UnaryOp,
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Couldn't resolve fields
    #[error("couldn't resolve fields in `{value}`")]
    #[diagnostic(code(rt::could_not_resolve_fields))]
    CouldNotResolveFields {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Couldn't call a value
    #[error("couldn't call `{value}`")]
    #[diagnostic(code(rt::could_not_call))]
    CouldNotCall {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Expected boolean value
    #[error("expected bool value. got `{value}`")]
    #[diagnostic(code(rt::expected_bool_value))]
    ExpectedBool {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Incorrect arity
    #[error("incorrect arity. expected {params} params got {args} args")]
    #[diagnostic(code(rt::incorrect_arity))]
    IncorrectArity {
        params: usize,
        args: usize,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Failed to find module
    #[error("failed to find module `{name}`")]
    #[diagnostic(code(rt::failed_to_find_module))]
    FailedToFindModule {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Bail
    #[error("bail: {text}")]
    #[diagnostic(code(rt::bail))]
    Bail {
        text: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("bail occurred here...")]
        span: SourceSpan,
    },
    /// Invalid channel
    #[error("channel `{value}` is invalid")]
    #[diagnostic(code(rt::invalid_chan))]
    InvalidChan {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("invalid channel here...")]
        span: SourceSpan,
    },
    /// Not a satellite
    #[error("`{value}` is not a satellite")]
    #[diagnostic(code(rt::not_a_satellite))]
    NotASatellite {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this is invalid...")]
        span: SourceSpan,
    },
    /// Unsafe satellite value
    #[error("value `{value}` can't be shared with satellite safely")]
    #[diagnostic(code(rt::unsafe_satl_value))]
    UnsafeSatlValue {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("unsafe satl value used here...")]
        span: SourceSpan,
    },
    /// Channel send error
    #[error("failed to send value to channel: {error}")]
    #[diagnostic(code(rt::chan_send_error))]
    ChanSendError {
        error: crossbeam::channel::SendError<ChanValue>,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("fail occurred here...")]
        span: SourceSpan,
    },
    /// Channel recv error
    #[error("failed to recv value from channel: {error}")]
    #[diagnostic(code(rt::chan_recv_error))]
    ChanRecvError {
        error: crossbeam::channel::RecvError,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("fail occurred here...")]
        span: SourceSpan,
    },
}
