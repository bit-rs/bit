/// Imports
use crate::{atom::TypeHint, expr::Expr};
use bit_common::span::Span;
use ecow::EcoString;

/// Statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    /// Let statement
    Let(Span, EcoString, Option<TypeHint>, Expr),

    /// Assignment statement
    VarAssign(Span, Expr, Expr),

    /// Expression statement without trailing semicolon
    Expr(Expr),

    /// Represents semi colon expression
    Semi(Expr),
}