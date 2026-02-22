/// Imports
use crate::{atom::Mutability, expr::Expr, ty::Ty};
use common::token::Span;

/// For range
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Range {
    /// If range excludes last value
    ExcludeLast(Span, Expr, Expr),
    /// If range includes last value
    IncludeLast(Span, Expr, Expr),
}

/// Statement kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtKind {
    /// While statement
    While(Expr, Block),

    /// For statement (var, range, block)
    For(String, Range, Block),

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Let definition
    Let(String, Ty, Mutability, Expr),

    /// Return statement
    Return(Ty, Option<Expr>),

    /// Expr without trailing semi-colon
    Expr(Expr),

    /// Expr with trailing semi-colon
    Semi(Expr),
}

/// Implementation
impl StmtKind {
    /// Returns true if statement requires semicolon after it
    pub fn requires_semi(&self) -> bool {
        match self {
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Let(_, _, _, _)
            | StmtKind::Return(_, _)
            | StmtKind::Semi(_) => true,
            StmtKind::While(_, _) | StmtKind::For(_, _, _) | StmtKind::Expr(_) => false,
        }
    }
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
