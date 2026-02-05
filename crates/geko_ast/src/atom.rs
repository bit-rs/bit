/// Imports
use crate::stmt::Block;
use geko_lex::token::Span;

/// Assignment operator
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
    Not,    // !
    Eq,     // ==
    Ne,     // !=
    BitAnd, // &
    BitOr,  // |
    Xor,    // ^
}

/// Unary operator
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,  // -
    Bang, // !
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
}

/// Function
#[derive(Debug, Clone)]
pub struct Function {
    /// Function span
    pub span: Span,
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}
