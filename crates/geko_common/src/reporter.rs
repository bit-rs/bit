/// Imports
use crate::{diagnostic::Diagnostic, interner::Interner};

/// Errors reporter responsible for
/// pretty errors reporting.
pub struct Reporter {
    /// Interner for files
    interner: Interner,
}

/// Reporter implementation
impl Reporter {
    /// Reports error
    pub fn report<T: Diagnostic>(&self, error: T) {
        error.report(&self.interner);
    }
}
