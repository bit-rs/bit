/// Imports
use crate::{
    errors::LexError,
    token::{Span, Token, TokenKind},
};
use geko_common::bail;
use miette::NamedSource;
use std::{str::Chars, sync::Arc};

/// Represents lexer
pub struct Lexer<'s> {
    /// Current file source
    source: Arc<NamedSource<String>>,

    /// Lexer source
    src: Chars<'s>,

    /// Current and next
    idx: usize,
    current: Option<char>,
    next: Option<char>,
}

/// Implementation
impl<'s> Lexer<'s> {
    /// Creates new lexer
    pub fn new(file: Arc<NamedSource<String>>, source: &'s str) -> Self {
        let mut chars = source.chars();
        let (current, next) = (chars.next(), chars.next());
        Self {
            source: file,
            src: chars,
            current,
            next,
            idx: 0,
        }
    }

    fn advance(&mut self) {
        self.current = self.next.take();
        self.next = self.src.next();
        self.idx += 1;
    }

    /// Advances char and returns token
    fn advance_with(&mut self, tk: TokenKind, lexeme: &str) -> Token {
        self.advance();
        Token::new(
            Span(self.source.clone(), self.idx - 1..self.idx),
            tk,
            lexeme.to_string(),
        )
    }

    /// Advances char twice and returns token
    fn advance_twice_with(&mut self, tk: TokenKind, lexeme: &str) -> Token {
        self.advance();
        self.advance();
        Token::new(
            Span(self.source.clone(), self.idx - 2..self.idx),
            tk,
            lexeme.to_string(),
        )
    }

    /// Advances string
    fn advance_string(&mut self) -> Token {
        // Advancing `"`
        self.advance();
        let start = self.idx;
        // Text buffer
        let mut buffer = String::new();
        // Building string before reaching `"`
        while self.current != Some('"') {
            buffer.push(self.current.unwrap());
            self.advance();
            // Checking for end of file
            if self.is_eof() {
                bail!(LexError::UnclosedStringQuotes {
                    src: self.source.clone(),
                    span: (start..self.idx).into(),
                })
            }
        }
        // Advancing `"`
        self.advance();
        let end = self.idx;
        Token::new(
            Span(self.source.clone(), start..end),
            TokenKind::String,
            buffer,
        )
    }

    /// Advances number
    fn advance_number(&mut self) -> Token {
        let start = self.idx;
        // If number is float
        let mut is_float = false;
        // Text buffer
        let mut buffer = String::new();
        // Building number before reaching
        // non-digit char.
        while self.is_digit() && !self.is_eof() {
            buffer.push(self.current.unwrap());
            self.advance();
            // Checking for float dot
            if self.current == Some('.') {
                // If next is digit
                if self.next.map(|it| it.is_ascii_digit()).unwrap_or(false) {
                    // If already float
                    if is_float {
                        bail!(LexError::InvalidFloat {
                            src: self.source.clone(),
                            span: (start..self.idx).into(),
                        })
                    } else {
                        buffer.push('.');
                        self.advance();
                        is_float = true;
                    }
                }
                // If next dot
                else if self.next == Some('.') {
                    break;
                }
            }
        }
        let end = self.idx;
        Token::new(
            Span(self.source.clone(), start..end),
            TokenKind::Number,
            buffer,
        )
    }

    /// Token kind for id
    fn token_kind_for_id(value: &str) -> TokenKind {
        match value {
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "in" => TokenKind::In,
            "let" => TokenKind::Let,
            "use" => TokenKind::Use,
            "type" => TokenKind::Type,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "continue" => TokenKind::Continue,
            "break" => TokenKind::Break,
            "as" => TokenKind::As,
            "true" => TokenKind::Bool,
            "false" => TokenKind::Bool,
            "fn" => TokenKind::Fn,
            _ => TokenKind::Id,
        }
    }

    /// Advances id or keyword
    fn advance_id_or_kw(&mut self) -> Token {
        let start = self.idx;
        // Text buffer
        let mut buffer = String::new();
        // Building id before reaching
        // char that is not letter, not digit,
        // and not underscore.
        while (self.is_id_letter() || self.is_digit()) && !self.is_eof() {
            buffer.push(self.current.unwrap());
            self.advance();
        }
        let end = self.idx;
        Token::new(
            Span(self.source.clone(), start..end),
            Self::token_kind_for_id(&buffer),
            buffer,
        )
    }

    /// Skips comment
    fn skip_comment(&mut self) {
        // #
        self.advance();
        while self.current != Some('\n') {
            self.advance();
        }
    }

    /// Skips multiline comment
    fn skip_multiline_comment(&mut self) {
        // #[
        self.advance();
        self.advance();
        while !(self.current == Some(']') && self.next == Some('#')) {
            self.advance();
        }
        // ]#
        self.advance();
        self.advance();
    }

    /// Skips whitespaces
    fn skip_whitespaces(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    /// Skips comments
    fn skip_comments(&mut self) {
        while self.current == Some('#') {
            if self.next == Some('[') {
                self.skip_multiline_comment();
            } else {
                self.skip_comment();
            }
        }
    }

    /// Is whitespace
    fn is_whitespace(&mut self) -> bool {
        match self.current {
            Some(' ') | Some('\n') | Some('\t') | Some('\r') => true,
            _ => false,
        }
    }

    /// Is id letter
    fn is_id_letter(&mut self) -> bool {
        match self.current {
            Some(it) if it.is_ascii_alphabetic() || it == '_' => true,
            _ => false,
        }
    }

    /// Is digit
    fn is_digit(&mut self) -> bool {
        match self.current {
            Some(it) if it.is_ascii_digit() => true,
            _ => false,
        }
    }

    /// Is end of file
    fn is_eof(&mut self) -> bool {
        match self.current {
            Some(_) => false,
            None => true,
        }
    }
}

/// Iterator implementation
impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Skipping whitespaces
        self.skip_whitespaces();
        self.skip_comments();

        // Matching current and next
        match (self.current, self.next) {
            (Some('+'), Some('=')) => Some(self.advance_twice_with(TokenKind::PlusEq, "+=")),
            (Some('-'), Some('=')) => Some(self.advance_twice_with(TokenKind::MinusEq, "-=")),
            (Some('*'), Some('=')) => Some(self.advance_twice_with(TokenKind::StarEq, "*=")),
            (Some('/'), Some('=')) => Some(self.advance_twice_with(TokenKind::SlashEq, "/=")),
            (Some('%'), Some('=')) => Some(self.advance_twice_with(TokenKind::PercentEq, "%=")),
            (Some('&'), Some('=')) => Some(self.advance_twice_with(TokenKind::AmpersandEq, "&=")),
            (Some('|'), Some('=')) => Some(self.advance_twice_with(TokenKind::BarEq, "|=")),
            (Some('^'), Some('=')) => Some(self.advance_twice_with(TokenKind::CaretEq, "^=")),
            (Some('&'), Some('&')) => Some(self.advance_twice_with(TokenKind::DoubleAmp, "&&")),
            (Some('|'), Some('|')) => Some(self.advance_twice_with(TokenKind::DoubleBar, "||")),
            (Some('='), Some('=')) => Some(self.advance_twice_with(TokenKind::DoubleEq, "==")),
            (Some('!'), Some('=')) => Some(self.advance_twice_with(TokenKind::BangEq, "!=")),
            (Some('.'), Some('.')) => Some(self.advance_twice_with(TokenKind::DoubleDot, "..")),
            (Some('>'), Some('=')) => Some(self.advance_with(TokenKind::Ge, ">=")),
            (Some('<'), Some('=')) => Some(self.advance_with(TokenKind::Le, "<=")),
            (Some('&'), _) => Some(self.advance_with(TokenKind::Ampersand, "&")),
            (Some('|'), _) => Some(self.advance_with(TokenKind::Bar, "|")),
            (Some('^'), _) => Some(self.advance_with(TokenKind::Caret, "^")),
            (Some('%'), _) => Some(self.advance_with(TokenKind::Percent, "^")),
            (Some('+'), _) => Some(self.advance_with(TokenKind::Plus, "+")),
            (Some('-'), _) => Some(self.advance_with(TokenKind::Minus, "-")),
            (Some('*'), _) => Some(self.advance_with(TokenKind::Star, "*")),
            (Some('/'), _) => Some(self.advance_with(TokenKind::Slash, "/")),
            (Some('!'), _) => Some(self.advance_with(TokenKind::Bang, "!")),
            (Some('='), _) => Some(self.advance_with(TokenKind::Eq, "=")),
            (Some('>'), _) => Some(self.advance_with(TokenKind::Gt, ">")),
            (Some('<'), _) => Some(self.advance_with(TokenKind::Lt, "<")),
            (Some('.'), _) => Some(self.advance_with(TokenKind::Dot, ".")),
            (Some(','), _) => Some(self.advance_with(TokenKind::Comma, ",")),
            (Some('{'), _) => Some(self.advance_with(TokenKind::Lbrace, "{")),
            (Some('}'), _) => Some(self.advance_with(TokenKind::Rbrace, "}")),
            (Some('['), _) => Some(self.advance_with(TokenKind::Lbracket, "[")),
            (Some(']'), _) => Some(self.advance_with(TokenKind::Rbracket, "]")),
            (Some('('), _) => Some(self.advance_with(TokenKind::Lparen, "(")),
            (Some(')'), _) => Some(self.advance_with(TokenKind::Rparen, ")")),
            (Some(';'), _) => Some(self.advance_with(TokenKind::Semi, ";")),
            (Some('"'), _) => Some(self.advance_string()),
            (Some(ch), _) => {
                if self.is_digit() {
                    Some(self.advance_number())
                } else if self.is_id_letter() {
                    Some(self.advance_id_or_kw())
                } else {
                    bail!(LexError::UnexpectedChar {
                        ch,
                        src: self.source.clone(),
                        span: (self.idx..self.idx).into(),
                    })
                }
            }
            (_, _) => None,
        }
    }
}
