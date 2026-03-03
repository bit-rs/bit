/// Imports
use crate::{atom::Mutability, expr::Expr, ty::Ty};
use common::token::Span;

/// Statement kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtKind {
    /// Let definition
    Let(String, Ty, Mutability, Expr),

    /// Expr without trailing semi-colon
    Expr(Expr),

    /// Expr with trailing semi-colon
    Semi(Expr),
}

/// Represents statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

/// Represents statements block
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}
