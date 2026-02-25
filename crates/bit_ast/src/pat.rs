/// Imports
use crate::expr::Expr;
use bit_common::span::Span;
use ecow::EcoString;

/// Represents unwrap field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnwrapField {
    // `_`
    Wildcard(Span),
    // Field
    Field(Span, EcoString),
}

/// Represents unwrap pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pat {
    /// Represents enum fields unwrap pattern
    Unwrap(Span, Expr, Vec<UnwrapField>),
    /// Represents just enum variant pattern
    Variant(Span, Expr),
    /// Represents integer pattern, e.g `123`
    Int(Span, EcoString),
    /// Represents float pattern, e.g `1.34`
    Float(Span, EcoString),
    /// Represents bool pattern, e.g `true` / `false
    Bool(Span, EcoString),
    /// Represents string pattern, e.g "Hello, world!"
    String(Span, EcoString),
    /// Represents bind pattern
    BindTo(Span, EcoString),
    /// Represents wildcard pattern
    Wildcard,
    /// Represents or pattern
    Or(Box<Pat>, Box<Pat>),
}

/// Represents case
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Case {
    pub address: Span,
    pub pat: Pat,
    pub body: Expr,
}
