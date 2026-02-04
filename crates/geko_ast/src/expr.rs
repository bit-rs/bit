/// Imports
use crate::atom::{BinaryOp, Lit, UnaryOp};
use geko_lex::token::Span;

/// Range
pub enum Range {
    // x..y
    IncludeLast {
        span: Span,
        from: Expression,
        to: Expression,
    },
    // x..=y
    ExcludeLast {
        span: Span,
        from: Expression,
        to: Expression,
    },
}

/// Expression
pub enum Expression {
    // Literal
    Literal {
        span: Span,
        literal: Lit,
    },
    // Binary operation
    Bin {
        span: Span,
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // Unary operation
    Unary {
        span: Span,
        op: UnaryOp,
        value: Box<Expression>,
    },
    // Variable access
    Variable {
        span: Span,
        name: String,
    },
    // Field access
    Field {
        span: Span,
        name: String,
        container: Box<Expression>,
    },
    // Call expression
    Call {
        span: Span,
        args: Vec<Box<Expression>>,
    },
    /// Function expression
    Function {
        span: Span,
    },
}
