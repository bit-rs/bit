/// Imports
use crate::token::Span;

/// Lexer error
pub enum LexError {
    /// Unexpected char
    UnexpectedChar { span: Span },
    /// Unclosed string quotes
    UnclosedStringQuotes { span: Span },
    /// Invalid float
    InvalidFloat { span: Span },
}
