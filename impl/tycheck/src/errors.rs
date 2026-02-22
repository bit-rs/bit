/// Imports
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
