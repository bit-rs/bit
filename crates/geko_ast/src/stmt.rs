/// Imports
use crate::{
    atom::Function,
    expr::{Expression, Range},
};
use geko_lex::token::Span;

/// Statement
pub enum Statement {
    // While statement
    While {
        span: Span,
        condition: Expression,
        block: Block,
    },
    // If statement
    If {
        span: Span,
        then: Block,
        else_: Box<Statement>,
    },
    // For statement
    For {
        span: Span,
        var: String,
        range: Range,
        block: Block,
    },
    // Type declaration
    Type {
        span: Span,
        name: String,
        functions: Vec<Function>,
    },
}

/// Represents block
pub struct Block {
    statements: Vec<Statement>,
}
