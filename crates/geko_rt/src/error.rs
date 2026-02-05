/// Imports
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

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
    /// Variable is already defined
    #[error("variable `{name}` is already defined.")]
    #[diagnostic(code(rt::undefined_variable))]
    AlreadyDefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("redeclaration attempt here...")]
        span: SourceSpan,
    },
}
