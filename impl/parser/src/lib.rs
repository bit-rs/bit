/// Modules
mod atom;
#[allow(unused_assignments)]
mod errors;
mod item;
mod stmt;

/// Imports
use ast::{expr::Expr, item::Module};
use common::token::{Token, TokenKind};
use lexer::Lexer;
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
        Module {
            source: self.source.clone(),
            items,
        }
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

        if !self.check(close.clone()) {
            loop {
                items.push(parse_item(self));
                if self.check(sep.clone()) {
                    self.expect(sep.clone());
                    if self.check(close.clone()) {
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
            if self.check(sep.clone()) {
                self.expect(sep.clone());
            } else {
                break;
            }
        }

        items
    }

    /// Arguments parsing
    pub(crate) fn args(&mut self) -> Vec<Expr> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expr(),
        )
    }

    /// Single parameter parsing
    fn param(&mut self) -> String {
        let name = self.expect(TokenKind::Id).lexeme;
    }

    /// Variable parsing
    fn variable(&mut self) -> Expression {
        // parsing base identifier
        let start_span = self.peek().span.clone();
        let id = self.expect(TokenKind::Id).lexeme;

        // result node
        let mut result = Expression::Variable {
            span: start_span.clone(),
            name: id,
        };

        // checking for dots and parens
        loop {
            // checking for chain `a.b.c.d`
            if self.check(TokenKind::Dot) {
                self.bump();
                let id = self.expect(TokenKind::Id).lexeme;
                let end_span = self.prev().span.clone();
                result = Expression::Field {
                    span: start_span.clone() + end_span,
                    container: Box::new(result),
                    name: id,
                };
                continue;
            }
            // checking for call
            if self.check(TokenKind::Lparen) {
                let args = self.args();
                let end_span = self.prev().span.clone();
                result = Expression::Call {
                    span: start_span.clone() + end_span,
                    what: Box::new(result),
                    args,
                };
                continue;
            }
            // breaking cycle
            break;
        }
        result
    }

    /// Group expression parsing
    fn group(&mut self) -> Expression {
        self.expect(TokenKind::Lparen);
        let expr = self.expr();
        self.expect(TokenKind::Rparen);
        expr
    }

    /// Function parsing
    fn function(&mut self) -> Function {
        // Start span
        let start_span = self.peek().span.clone();

        // `fn` keyword
        self.expect(TokenKind::Fn);

        // Function name
        let name = self.expect(TokenKind::Id).lexeme;

        // Parsing params
        let params = self.params();

        // Parsing block
        let block = self.block();

        // End span
        let end_span = self.prev().span.clone();

        // Done
        Function {
            name,
            span: start_span + end_span,
            params,
            block,
        }
    }

    /// Atom expression parsing
    fn atom(&mut self) -> Expression {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Lparen => self.group(),
            TokenKind::Number => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Number(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::String => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::String(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Bool => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Bool(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Null => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Null,
                };
                self.bump();
                expr
            }
            TokenKind::Id => self.variable(),
            _ => bail!(ParseError::UnexpectedExprToken {
                got: tk.kind,
                src: self.source.clone(),
                span: tk.span.1.into(),
            }),
        }
    }

    /// Unary expression parsing
    fn unary_expr(&mut self) -> Expression {
        if self.check(TokenKind::Minus) || self.check(TokenKind::Bang) {
            let start_span = self.peek().span.clone();
            let op = self.bump();
            let value = self.unary_expr();
            let end_span = self.prev().span.clone();
            return Expression::Unary {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Minus => UnaryOp::Neg,
                    TokenKind::Bang => UnaryOp::Bang,
                    _ => unreachable!(),
                },
                value: Box::new(value),
            };
        }
        self.atom()
    }

    /// Factor expression parsing
    fn factor_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.unary_expr();
        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let op = self.bump();
            let right = self.unary_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Star => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
                    TokenKind::Percent => BinaryOp::Mod,
                    _ => unreachable!(),
                },
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// Term expression parsing
    fn term_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.factor_expr();
        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let op = self.bump();
            let right = self.factor_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Sub,
                    _ => unreachable!(),
                },
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// Compare expression parsing
    fn compare_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.term_expr();
        while self.check(TokenKind::Ge)
            || self.check(TokenKind::Gt)
            || self.check(TokenKind::Le)
            || self.check(TokenKind::Lt)
        {
            let op = self.bump();
            let right = self.term_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Ge => BinaryOp::Ge,
                    TokenKind::Gt => BinaryOp::Gt,
                    TokenKind::Le => BinaryOp::Le,
                    TokenKind::Lt => BinaryOp::Lt,
                    _ => unreachable!(),
                },
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// Equality expression parsing
    fn equality_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.compare_expr();
        while self.check(TokenKind::DoubleEq) || self.check(TokenKind::BangEq) {
            let op = self.bump();
            let right = self.compare_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::DoubleEq => BinaryOp::Eq,
                    TokenKind::BangEq => BinaryOp::Ne,
                    _ => unreachable!(),
                },
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// `Bitwise and` expression parsing
    fn bitwise_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();
        while self.check(TokenKind::Ampersand) {
            self.bump();
            let right = self.equality_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinaryOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// `Bitwise xor` expression parsing
    fn bitwise_xor_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_and_expr();
        while self.check(TokenKind::Caret) {
            self.bump();
            let right = self.bitwise_and_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinaryOp::Xor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    /// `Bitwise or` expression parsing
    fn bitwise_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_xor_expr();
        while self.check(TokenKind::Bar) {
            self.bump();
            let right = self.bitwise_xor_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinaryOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// `Logical and` expression parsing
    fn logical_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_or_expr();
        while self.check(TokenKind::DoubleAmp) {
            self.bump();
            let right = self.bitwise_or_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// `Logical or` expression parsing
    fn logical_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_and_expr();
        while self.check(TokenKind::DoubleBar) {
            self.bump();
            let right = self.logical_and_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// Parses expression
    fn expr(&mut self) -> Expression {
        self.logical_or_expr()
    }

    /// Checks token match
    pub(crate) fn check(&self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
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
