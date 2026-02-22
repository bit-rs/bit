/// Modules
mod atom;
#[allow(unused_assignments)]
mod errors;
mod expr;
mod item;
mod stmt;

/// Imports
use crate::errors::ParseError;
use ast::item::Module;
use common::token::{Token, TokenKind};
use lexer::Lexer;
use macros::bail;
use miette::NamedSource;
use std::sync::Arc;

/// Parser is struct that converts a stream of tokens
/// produced by the lexer into an abstract syntax tree (AST).
pub struct Parser<'s> {
    /// Named source of the file
    pub(crate) source: Arc<NamedSource<String>>,

    /// Lexer used to iterate over tokens
    lexer: Lexer<'s>,

    /// Previously consumed token
    /// (useful for spans and error reporting)
    previous: Option<Token>,

    /// Current token under inspection
    pub(crate) current: Option<Token>,

    /// Lookahead token
    /// (used for predictive parsing)
    next: Option<Token>,
}

/// Implementation
impl<'s> Parser<'s> {
    /// Creates new parser
    pub fn new(source: Arc<NamedSource<String>>, mut lexer: Lexer<'s>) -> Self {
        let current = lexer.next();
        let next = lexer.next();
        Self {
            source,
            lexer,
            previous: None,
            current,
            next,
        }
    }

    /// Parses module
    pub fn parse(&mut self) -> Module {
        let mut items = Vec::new();
        while self.current.is_some() {
            items.push(self.item())
        }
        Module { items }
    }

    /// Sep by parsing
    pub(crate) fn sep_by<T>(
        &mut self,
        open: TokenKind,
        close: TokenKind,
        sep: TokenKind,
        mut parse_item: impl FnMut(&mut Self) -> T,
    ) -> Vec<T> {
        let mut items = Vec::new();
        self.expect(open);

        if !self.check(close) {
            loop {
                items.push(parse_item(self));
                if self.check(sep) {
                    self.expect(sep);
                    if self.check(close) {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        self.expect(close);
        items
    }

    /// Sep by parsing without open or close tokens
    pub(crate) fn sep_by_2<T>(
        &mut self,
        sep: TokenKind,
        mut parse_item: impl FnMut(&mut Self) -> T,
    ) -> Vec<T> {
        let mut items = Vec::new();

        loop {
            items.push(parse_item(self));
            if self.check(sep) {
                self.expect(sep);
            } else {
                break;
            }
        }

        items
    }

    /// Checks, does token match or not
    pub(crate) fn check(&self, tk: TokenKind) -> bool {
        self.current.as_ref().map(|x| x.kind == tk).unwrap_or_default()
    }

    /// Retrieves current token
    pub(crate) fn peek(&self) -> &Token {
        match &self.current {
            Some(tk) => tk,
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Retrieves previous token
    pub(crate) fn prev(&self) -> &Token {
        match &self.previous {
            Some(tk) => tk,
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Expects token with kind
    pub(crate) fn expect(&mut self, tk: TokenKind) -> Token {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    self.bump()
                } else {
                    bail!(ParseError::UnexpectedToken {
                        got: it.kind.clone(),
                        expected: tk,
                        src: self.source.clone(),
                        span: it.span.1.clone().into(),
                        prev: self.prev().span.1.clone().into(),
                    })
                }
            }
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Advances current token
    pub(crate) fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
