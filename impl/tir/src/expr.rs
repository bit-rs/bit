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

/// Resolution definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Def {
    /// ADT definition
    Adt(Id<AdtDef>),

    /// Function definition
    Fn(Id<FnDef>),
}

/// Expression resolution
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Res {
    /// Some definition
    Def(Def),

    /// Variant of ADT definition
    Variant(Id<AdtDef>, String),

    /// Local or field type
    Ty(Ty),
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
    Call(Box<Expr>, Vec<Expr>, Res),

    /// Id expr (e.g, `foo`)
    Id(String, Res),

    /// Field expr (e.g, `wibble.wobble`)
    Field(Box<Expr>, String, Res),

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
