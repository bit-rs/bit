/// Imports
use crate::stmt::Block;
use bit_lex::token::Span;
use std::fmt::Display;

/// Assignment operator
#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign, // =
    Add,    // +=
    Sub,    // -=
    Mul,    // *=
    Div,    // /=
    Mod,    // %=
    BitAnd, // &=
    BitOr,  // |=
    Xor,    // ^=
}

/// Binary operator
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    And,    // &&
    Or,     // ||
    Gt,     // >
    Ge,     // >=
    Lt,     // <
    Le,     // <=
    Eq,     // ==
    Ne,     // !=
    BitAnd, // &
    BitOr,  // |
    Xor,    // ^
}

/// Display implementation
impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
            BinaryOp::BitAnd => write!(f, "&"),
            BinaryOp::BitOr => write!(f, "|"),
            BinaryOp::Xor => write!(f, "^"),
        }
    }
}

/// Unary operator
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,  // -
    Bang, // !
}

/// Display implementation
impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}

/// Literal
#[derive(Debug, Clone)]
pub enum Lit {
    /// Number literal
    Number(String),
    /// String literal
    String(String),
    /// Bool literal
    Bool(String),
    /// Null literal
    Null,
}

/// Function item
#[derive(Debug, Clone)]
pub struct Function {
    /// Function span
    pub span: Span,
    /// Function signature span
    pub sign_span: Span,
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}

/// Satellite item
#[derive(Debug, Clone)]
pub struct Satellite {
    /// Satellite span
    pub span: Span,
    /// Satellite signature span
    pub sign_span: Span,
    /// Satellite name
    pub name: String,
    /// Satellite channel
    pub chan: String,
    /// Satellite block
    pub block: Block,
}

/// Type item
#[derive(Debug, Clone)]
pub struct Type {
    /// Type span
    pub span: Span,
    /// Type name span
    pub name_span: Span,
    /// Type name
    pub name: String,
    /// Type methods
    pub methods: Vec<Function>,
}
