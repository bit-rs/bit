/// Modules
pub mod ty;

/// Imports
use crate::cx::icx::InferCx;
use ast::expr::{BinOp, UnOp};
use common::token::Span;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Typeck error
#[derive(Error, Diagnostic, Debug)]
pub enum TypeckError {
    /// Unsigned int negation
    #[error("can't negate unsigned integer `{ty}`")]
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
        ty: String,
        op: UnOp,
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

    /// Type mismatch
    #[error("type mismatch: expected `{t1}`, found `{t2}`")]
    #[diagnostic(code(typeck::type_mismatch))]
    TypeMismatch {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
        t1: String,
        t2: String,
    },

    /// Rigid type mismatch
    #[error("can't unify generic type `{ty}` with a concrete type")]
    #[diagnostic(code(typeck::rigid_mismatch))]
    RigidMismatch {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
        ty: String,
    },

    /// Infinite type
    #[error("infinite type detected")]
    #[diagnostic(code(typeck::infinite_type))]
    InfiniteType {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },

    /// Unresolved name
    #[error("can't find value with name `{name}`")]
    UnresolvedName {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this name is not found in this scope")]
        span: SourceSpan,
        name: String,
    },

    /// Unresolved type
    #[error("can't find type with name `{name}`")]
    UnresolvedType {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this type is not found")]
        span: SourceSpan,
        name: String,
    },

    /// Unresolved field
    #[error("can't find field with name `{name}`")]
    UnresolvedField {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this field is not found")]
        span: SourceSpan,
        name: String,
    },

    /// Can not call
    #[error("can't call value `{ty}`")]
    CanNotCall {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this is invalid")]
        span: SourceSpan,
        ty: String,
    },

    /// Arity missmatch
    #[error("arity missmatch. expected `{expected}`, got `{got}`")]
    ArityMissmatch {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this call is not valid")]
        span: SourceSpan,
        expected: usize,
        got: usize,
    },

    /// Already defined
    #[error("value `{binding}` already defined in this scope")]
    AlreadyDefined {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this binding is invalid")]
        span: SourceSpan,
        binding: String,
    },
}

/// An `IntoDiagnostic` trait, used to convert error
/// into `TypeckError`. Provides reference to `InferCx`
/// and span, where error happened
///
pub trait IntoDiagnostic {
    fn into_diag(&self, icx: &InferCx, span: Span) -> TypeckError;
}
