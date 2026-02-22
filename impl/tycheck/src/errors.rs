/// Imports
use ast::expr::{BinOp, UnOp};
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;
use tir::ty::Ty;

/// Types error
#[derive(Debug)]
pub enum TypeError {
    /// Types missmatch
    Mismatch(Ty, Ty),
    /// Attempt to unify rigid and something else
    RigidMismatch(Ty),
    /// Infinite type (occurs check failure)
    InfiniteType,
}

/// Typeck error
#[derive(Error, Diagnostic, Debug)]
pub enum TypeckError {
    /// Unsigned int negation
    #[error("could not negate unsigned integer `{ty}`")]
    #[diagnostic(code(typeck::uint_negation))]
    UIntNegation {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
        ty: String,
    },

    /// Invalid unary operation
    #[error("invalid unary op `{op:?}` on expr with ty `{ty}`")]
    #[diagnostic(code(typeck::invalid_unary_op))]
    InvalidUnaryOp {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
        op: UnOp,
        ty: String,
    },

    /// Invalid unary operation
    #[error("invalid unary op `{op:?}` on expr-s with ty-s `{t1}` and `{t2}`")]
    #[diagnostic(code(typeck::invalid_bin_op))]
    InvalidBinOp {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
        op: BinOp,
        t1: String,
        t2: String,
    },
}
