/// Imports
use ecow::EcoString;
use bit_common::span::Span;

/// Token kind
#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
#[allow(dead_code)]
pub enum TokenKind {
    Let,       // let
    Fn,        // fn
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Or,        // || | or
    And,       // && | and
    Bar,       // |
    Ampersand, // &
    AddAssign, // +=
    SubAssign, // -=
    MulAssign, // *-
    DivAssign, // /=
    AndAssign, // &=
    OrAssign,  // |=
    XorAssign, // ^=
    Lparen,    // (
    Rparen,    // )
    Lbrace,    // {
    Rbrace,    // }
    Eq,        // ==
    NotEq,     // !=
    Text,      // 'text'
    Number,    // 1234567890.0123456789
    Assign,    // =
    Id,        // variable id
    Comma,     // ,
    If,        // if
    Bool,      // bool
    Loop,      // loop
    Type,      // type
    Enum,      // enum
    Dot,       // .
    Range,     // ..
    Greater,   // >
    Less,      // <
    GreaterEq, // >=
    LessEq,    // <=
    Concat,    // <>
    Elif,      // elif
    Else,      // else
    Use,       // use
    Lbracket,  // [
    Rbracket,  // ]
    Colon,     // :
    Semicolon, // ;
    Bang,      // !
    Wildcard,  // _
    In,        // in
    Unit,      // unit
    As,        // as
    Pub,       // pub
    Match,     // match
    Arrow,     // arrow
    Extern,    // extern
    For,       // for
    Panic,     // panic
    Todo,      // todo
    Const,     // const
}

/// Token structure
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Token {
    pub tk_type: TokenKind,
    pub value: EcoString,
    pub address: Span,
}

/// Token implementation
impl Token {
    /// Creates token from tk_type, value, address
    pub fn new(tk_type: TokenKind, value: EcoString, address: Span) -> Token {
        Token {
            tk_type,
            value,
            address,
        }
    }
}
