/// Imports
use std::ops::{Add, Range};

use camino::Utf8PathBuf;
use id_arena::Id;

/// Represents keyword
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Keyword {
    For,
    While,
    In,
    Let,
    Use,
    Type,
    If,
    Else,
    Return,
    Continue,
    Break,
    As,
}

/// Punctuation token
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Punctuator {
    Comma,       // ,
    Dot,         // .
    Lbrace,      // {
    Rbrace,      // }
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
    Amp,         // &
    Bang,        // !
    Bar,         // |
    Eq,          // =
    Ge,          // >=
    Le,          // <=
    Gt,          // >
    Lt,          // <
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
    DoubleDot,   // ..
}

/// Represents token kind
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Punctuation(Punctuator),
    Number(String),
    String(String),
    Id(String),
}

/// Represents token
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token(pub Span, pub TokenKind);

/// Represents span
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Span(pub Id<Utf8PathBuf>, pub Range<usize>);

/// Add implementation
impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        // Checking that files are same
        if self.0 != rhs.0 {
            panic!("attemp to perform `+` operation on two spans from different files.")
        }
        // Calculating new span range
        let start = self.1.start.min(rhs.1.start);
        let end = self.1.end.min(rhs.1.end);
        Span(self.0, start..end)
    }
}
