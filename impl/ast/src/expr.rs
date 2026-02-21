/// Imports
use crate::{atom::TypeHint, stmt::Block};
use common::token::Span;

/// Literal
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    /// Int
    Int(String),

    /// Float
    Float(String),

    /// String
    String(String),

    /// Bool
    Bool(String),
}

/// Unary operation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnOp {
    // -
    Neg,
    
    // !
    Bang,
    
    // *
    Deref,
    
    // &
    Ref,
    
    // &mut
    MutRef
}

/// Binary operation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    // +
    Plus,
    
    // -
    Sub,
    
    // *
    Mul,
    
    // /
    Div,
    
    // %
    Mod,
    
    // &&
    And,
    
    // ||
    Or,
    
    // &
    BitAnd,
    
    // |
    BitOr,
    
    // ^
    Xor,
    
    // ==
    Eq,
    
    // !=
    Ne,
    
    // >=
    Ge,
    
    // <=
    Le,
    
    // >
    Gt,
    
    // <
    Lt,
    
    // <>
    Concat,
}

/// Expression kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    /// Literal (lit)
    Lit(Lit),

    /// Unary opreation (unary, expr)
    Unary(UnOp, Box<Expr>),

    /// Binary operation (bin, lhs, rhs)
    Bin(BinOp, Box<Expr>, Box<Expr>),

    /// If operation (condition, then, else)
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Call expr (e.g `foo(arg, arg, ..n)` )
    Call(Box<Expr>, Vec<Expr>),

    /// Id expr (e.g, `foo`)
    Id(String),

    /// Field expr (e.g, `wibbe.wobble`)
    Field(Box<Expr>, String),

    /// Cast expr (e.g. `foo as f64`)
    Cast(Box<Expr>, TypeHint),

    /// Closure expr (e.g `|param, param, ..n| ...`)
    Closure(Vec<String>, Box<Expr>),

    /// Block
    Block(Box<Block>),
}

/// Expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
