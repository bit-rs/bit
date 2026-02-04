/// Imports
use crate::interner::Interner;

/// Diagnostic trait
pub trait Diagnostic {
    /// Should report error
    fn report(self, interner: &Interner);
}
