/// Imports
use crate::{expr::Expr, pat::Pat};
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
    Let(String, Expr),
    
    /// Assignment
    Assign(Pat, Expr),

    /// Return statement
    Return(Option<Expr>),

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
}
