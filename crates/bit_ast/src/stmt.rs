/// Imports
use crate::{
    atom::{AssignOp, Function, Satellite, Type},
    expr::{Expression, Range},
};
use bit_lex::token::Span;

/// Usage
#[derive(Debug, Clone)]
pub enum UsageKind {
    // As `name`
    As(String),
    // For `items`
    For(Vec<String>),
    // For every item
    All,
    // Just import
    Just,
}

/// Statement
#[derive(Debug, Clone)]
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
        condition: Expression,
        then: Block,
        else_: Option<Box<Statement>>,
    },
    // For statement
    For {
        span: Span,
        var: String,
        range: Range,
        block: Block,
    },
    // Type declaration
    Type(Type),
    // Function declaration
    Function(Function),
    // Satellite declaration
    Satl(Satellite),
    // Let declaration
    Let {
        span: Span,
        name: String,
        value: Expression,
    },
    // Assignment declaration
    Assign {
        span: Span,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Field assignment declaration
    Set {
        span: Span,
        container: Expression,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Return statement
    Return {
        span: Span,
        expr: Option<Expression>,
    },
    // Continue statement
    Continue(Span),
    // Break statement
    Break(Span),
    // Expr
    Expr(Expression),
    // Block
    Block(Box<Block>),
    // Use statement
    Use {
        span: Span,
        path: String,
        kind: UsageKind,
    },
    // Send statement
    Send {
        span: Span,
        what: Expression,
        to: Expression,
    },
    // Bail statement
    Bail {
        span: Span,
        message: Expression,
    },
}

/// Implementation
impl Statement {
    /// Is statement requires semicolon after it?
    pub fn requires_semi(&self) -> bool {
        match self {
            // Without semicolon
            Statement::While { .. }
            | Statement::If { .. }
            | Statement::For { .. }
            | Statement::Type { .. }
            | Statement::Function(_)
            | Statement::Block(_)
            | Statement::Satl(_) => false,
            // With semicolon
            Statement::Let { .. }
            | Statement::Assign { .. }
            | Statement::Continue(_)
            | Statement::Break(_)
            | Statement::Return { .. }
            | Statement::Expr(_)
            | Statement::Set { .. }
            | Statement::Use { .. }
            | Statement::Bail { .. }
            | Statement::Send { .. } => true,
        }
    }
}

/// Represents block
#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
}
