/// Imports
use bit_common::span::Span;
use std::fmt::Debug;

/// Represents token kind
#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum TokenKind {
    Import,      // `import` keyword
    Struct,      // `struct` keyword
    Enum,        // `enum` keyword
    Type,        // `type` keyword
    Let,         // `let` keyword
    If,          // `if` keyword
    Else,        // `else` keyword
    Fn,          // `fn` keyword
    None,        // `none` keyword
    Pub,         // `pub` keyword
    Comma,       // ,
    Dot,         // .
    Lparen,      // (
    Rparen,      // )
    Lbracket,    // [
    Rbracket,    // ]
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Ampersand,   // &
    Bang,        // !
    Bar,         // |
    Eq,          // =
    Ge,          // >=
    Le,          // <=
    Gt,          // >
    Lt,          // <
    Colon,       // :
    Semi,        // ;
    DoubleEq,    // ==
    DoubleBar,   // ||
    DoubleAmp,   // &&
    BangEq,      // !=
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    CaretEq,     // ^=
    PercentEq,   // %=
    BarEq,       // |=
    AmpersandEq, // &=
    Arrow,       // ->
    DoubleDot,   // ..
    Number,      // any number
    String,      // "quoted text"
    Id,          // identifier
    Bool,        // bool
}

/// Represents token
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub lexeme: String,
}

/// Implementation
impl Token {
    /// Creates new token
    pub fn new(span: Span, kind: TokenKind, lexeme: String) -> Self {
        Self { span, kind, lexeme }
    }
}
