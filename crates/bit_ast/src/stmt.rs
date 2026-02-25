/// Imports
use crate::{atom::TypeHint, expr::Expr};
use bit_common::span::Span;

/// Statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    /// Let statement
    Let(Span, String, Option<TypeHint>, Expr),

    /// Expression statement without trailing semicolon
    Expr(Expr),

    /// Represents semi colon expression
    Semi(Expr),
}
