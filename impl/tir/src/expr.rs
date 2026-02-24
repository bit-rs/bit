/// Imports
use crate::{
    atom::Param,
    def::{AdtDef, FnDef},
    stmt::Block,
    ty::Ty,
};
use ast::expr::{AssignOp, BinOp, UnOp};
use common::token::Span;
use id_arena::Id;

/// Literal
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    /// Number
    Number(String),

    /// String
    String(String),

    /// Char
    Char(char),

    /// Bool
    Bool(bool),
}

/// Expression resolution
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Resolution {
    Adt(Id<AdtDef>),
    Fn(Id<FnDef>),
    Variant(Id<AdtDef>, usize),
    Local(Ty),
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
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    /// Call expr (e.g `foo(arg, arg, ..n)` )
    Call(Box<Expr>, Vec<Expr>),

    /// Id expr (e.g, `foo`)
    Id(String),

    /// Field expr (e.g, `wibbe.wobble`)
    Field(Box<Expr>, String),

    /// Cast expr (e.g. `foo as f64`)
    Cast(Box<Expr>, Ty),

    /// Closure expr (e.g `|param, param, ..n| ...`)
    Closure(Vec<Param>, Box<Expr>),

    /// Assignment expr (e.g `a = b`)
    Assign(Box<Expr>, AssignOp, Box<Expr>),

    /// Block
    Block(Box<Block>),
}

/// Expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Ty,
}
