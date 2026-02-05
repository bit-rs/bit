/// Imports
use crate::value::Value;
use geko_ast::atom::{BinaryOp, UnaryOp};
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
    #[error("variable `{name}` is not defined.")]
    #[diagnostic(code(rt::undefined_variable))]
    UndefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable access here...")]
        span: SourceSpan,
    },
    /// Undefined field
    #[error("field `{name}` is not defined.")]
    #[diagnostic(code(rt::undefined_field))]
    UndefinedField {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("field access here...")]
        span: SourceSpan,
    },
    /// Variable is already defined
    #[error("variable `{name}` is already defined.")]
    #[diagnostic(code(rt::already_defined_variable))]
    AlreadyDefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("redeclaration attempt here...")]
        span: SourceSpan,
    },
    /// Invalid binary op
    #[error("could not use `{op}` with {a} and {b}")]
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
    #[error("could not use `{op}` with {value}")]
    #[diagnostic(code(rt::invalid_unary_op))]
    InvalidUnaryOp {
        op: UnaryOp,
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Could not resolve field
    #[error("could not resolve fields in {value}")]
    #[diagnostic(code(rt::could_not_resolve_fields))]
    CouldNotResolveFields {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Could not call value
    #[error("could not call {value}")]
    #[diagnostic(code(rt::could_not_call))]
    CouldNotCall {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Expected boolean value
    #[error("expected bool value. got {value}")]
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
}
