/// Imports
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Lexer error
#[derive(Error, Diagnostic, Debug)]
pub enum LexError {
    /// Unexpected char
    #[error("unexpected character \"{ch}\".")]
    #[diagnostic(code(lex::unexpected_char))]
    UnexpectedChar {
        ch: char,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("try to remove this character.")]
        span: SourceSpan,
    },
    /// Unclosed string quotes
    #[error("found unclosed string quotes.")]
    #[diagnostic(code(lex::unclosed_string_quotes))]
    UnclosedStringQuotes {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("close string quotes by appending missed quote `\"`.")]
        span: SourceSpan,
    },
    /// Invalid float
    #[error("invalid float number.")]
    #[diagnostic(code(lex::invalid_float_number))]
    InvalidFloat {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this float number seems to be invalid.")]
        span: SourceSpan,
    },
}
