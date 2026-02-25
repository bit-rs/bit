/// Imports
use crate::{
    atom::{BinOp, UnOp},
    pat::Case,
    stmt::Stmt,
};
use bit_common::span::Span;
use ecow::EcoString;

/// Represents literal
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    /// Integer number literal
    Int(EcoString),

    /// Floating-point number literal
    Float(EcoString),

    /// String literal
    String(EcoString),

    /// Boolean literal
    Bool(bool),
}

/// Represents expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// Literal expression
    Lit(Span, Lit),

    /// Represents todo expression (e.g `todo as "simple todo"`)
    Todo(Span, Option<EcoString>),

    /// Represents panic expression (e.g `panic as "simple panic"`)
    Panic(Span, Option<EcoString>),

    /// Represents unary expression
    Unary(Span, Box<Expr>, UnOp),

    /// Represents binary expression
    Bin(Span, Box<Expr>, Box<Expr>, BinOp),

    /// Represents if expression (cond, then, else)
    If(Span, Box<Expr>, Box<Expr>, Box<Expr>),

    /// Represents variable access
    Var(Span, EcoString),

    /// Represents field access
    Suffix(Span, Box<Expr>, EcoString),

    /// Represents call expression
    Call(Span, Box<Expr>, Vec<Expr>),

    /// Represents anonymous function expression
    Function(Span, Vec<String>, Box<Expr>),

    /// Represents match expression
    Match(Span, Box<Expr>, Vec<Case>),

    /// Represents paren expression
    Paren(Span, Box<Expr>),

    /// Block expression
    Block(Span, Vec<Stmt>),
}

/// Implementation
impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Lit(span, ..) => span.clone(),
            Expr::Panic(span, ..) => span.clone(),
            Expr::Todo(span, ..) => span.clone(),
            Expr::Bin(span, ..) => span.clone(),
            Expr::Unary(span, ..) => span.clone(),
            Expr::If(span, ..) => span.clone(),
            Expr::Var(span, ..) => span.clone(),
            Expr::Suffix(span, ..) => span.clone(),
            Expr::Call(span, ..) => span.clone(),
            Expr::Function(span, ..) => span.clone(),
            Expr::Match(span, ..) => span.clone(),
            Expr::Paren(span, ..) => span.clone(),
            Expr::Block(span, ..) => span.clone(),
        }
    }
}
