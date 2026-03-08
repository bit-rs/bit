/// Imports
use crate::{
    cx::icx::InferCx,
    errors::{IntoDiagnostic, TypeckError},
};
use common::token::Span;
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

/// Into diagnostic impl
impl IntoDiagnostic for TypeError {
    fn into_diag(&self, icx: &InferCx, span: Span) -> TypeckError {
        match self {
            TypeError::Mismatch(t1, t2) => TypeckError::TypeMismatch {
                src: span.0,
                span: span.1.into(),
                t1: icx.pretty(&t1),
                t2: icx.pretty(&t2),
            },
            TypeError::RigidMismatch(ty) => TypeckError::RigidMismatch {
                src: span.0,
                span: span.1.into(),
                ty: icx.pretty(&ty),
            },
            TypeError::InfiniteType => TypeckError::InfiniteType {
                src: span.0,
                span: span.1.into(),
            },
        }
    }
}
