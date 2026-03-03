/// Imports
use crate::cx::InferCx;
use ast::expr::{BinOp, UnOp};
use common::token::Span;
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
        #[label("not found in this scope")]
        span: SourceSpan,
        name: String,
    },

    /// Unresolved field
    #[error("can't find field with name `{name}`")]
    UnresolvedField {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("field not found")]
        span: SourceSpan,
        name: String,
    },

    /// Can not call
    #[error("can't call value `{ty}`")]
    CanNotCall {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("field not found")]
        span: SourceSpan,
        name: String,
    },

    /// Failed to infer resolution
    #[error("failed to resolve item")]
    Unresolved {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this is not found")]
        span: SourceSpan,
    },
}

/// Into diagnostic impl
impl TypeError {
    pub fn into_diagnostic(&self, icx: &InferCx, span: Span) -> TypeckError {
        match self {
            TypeError::Mismatch(t1, t2) => TypeckError::TypeMismatch {
                src: span.0,
                span: span.1.into(),
                t1: icx.print_ty(&t1),
                t2: icx.print_ty(&t2),
            },
            TypeError::RigidMismatch(ty) => TypeckError::RigidMismatch {
                src: span.0,
                span: span.1.into(),
                ty: icx.print_ty(&ty),
            },
            TypeError::InfiniteType => TypeckError::InfiniteType {
                src: span.0,
                span: span.1.into(),
            },
        }
    }
}
