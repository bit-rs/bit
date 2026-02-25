/// Imports
use miette::NamedSource;
use std::{
    fmt::Debug,
    ops::{Add, Range},
    sync::Arc,
};

/// Address structure
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Span {
    pub source: Arc<NamedSource<String>>,
    pub span: Range<usize>,
}

/// Address implementation
impl Span {
    /// New address with column
    pub fn new(source: Arc<NamedSource<String>>, at: usize) -> Span {
        Span {
            source,
            span: at..at,
        }
    }
    /// New address with span
    pub fn span(source: Arc<NamedSource<String>>, span: Range<usize>) -> Span {
        Span { source, span }
    }
}

/// Debug implementation
impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address({}..{})", self.span.start, self.span.end)
    }
}

/// Add implementation
impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        if self.source != rhs.source {
            panic!("address sources missmatched.")
        }
        Span::span(self.source, self.span.start..rhs.span.end)
    }
}
