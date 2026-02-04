/// Import
use geko_ast::{
    atom::{BinaryOp, Lit, UnaryOp},
    expr::Expression,
};
use geko_lex::{
    lexer::Lexer,
    token::{Punctuator, Span, Token, TokenKind},
};

/// Parser used to parse token
/// into the abstract syntax tree
pub struct Parser<'s> {
    /// Represen
    lexer: Lexer<'s>,
    /// Previous token
    previous: Option<Token>,
    /// Current token
    current: Option<Token>,
    /// Next token
    next: Option<Token>,
}

/// Implementation
impl<'s> Parser<'s> {
    /// Creates new parser
    pub fn new(mut lexer: Lexer<'s>) -> Self {
        let current = lexer.next();
        let next = lexer.next();
        Self {
            lexer,
            previous: None,
            current,
            next,
        }
    }

    /// Variable parsing
    fn variable(&mut self) -> Expression {
        // parsing base identifier
        let start_span = self.peek().span.clone();
        let variable = self.bump();

        // result node
        let mut result = Expression::Variable {
            span: start_span,
            name: variable.,
        };

        // checking for dots and parens
        loop {
            // checking for chain `a.b.c.d`
            if self.is_match(TokenKind::Punctuation(Punctuator::Dot)) {
                self.bump();
                let variable = self.consume(TokenKind::Id).clone();
                result = Expression::SuffixVar {
                    location: variable.address,
                    container: Box::new(result),
                    name: variable.value,
                };
                continue;
            }
            // checking for call
            if self.check(TokenKind::Lparen) {
                let args = self.args();
                let span_end = self.previous().address.clone();
                result = Expression::Call {
                    location: span_start.clone() + span_end,
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

    /// Atom expression parsing
    fn atom(&mut self) -> Expression {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Punctuation(punctuator) => {}
            TokenKind::Number(num) => Expression::Literal {
                span: tk.span,
                literal: Lit::Number(num),
            },
            TokenKind::String(val) => Expression::Literal {
                span: tk.span,
                literal: Lit::String(val),
            },
            TokenKind::Id(_) => self.variable(),
            _ => panic!("unexpected token."),
        }
    }

    /// Unary expression parsing
    fn unary_expr(&mut self) -> Expression {
        if self.is_match(TokenKind::Punctuation(Punctuator::Minus))
            || self.is_match(TokenKind::Punctuation(Punctuator::Bang))
        {
            let start_span = self.peek().span.clone();
            let op = self.bump();
            let value = self.atom();
            let end_span = self.prev().span.clone();
            return Expression::Unary {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Punctuation(Punctuator::Minus) => UnaryOp::Neg,
                    TokenKind::Punctuation(Punctuator::Bang) => UnaryOp::Bang,
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
        while self.is_match(TokenKind::Punctuation(Punctuator::Star))
            || self.is_match(TokenKind::Punctuation(Punctuator::Slash))
        {
            let op = self.bump();
            let right = self.unary_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Punctuation(Punctuator::Star) => BinaryOp::Mul,
                    TokenKind::Punctuation(Punctuator::Slash) => BinaryOp::Div,
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
        while self.is_match(TokenKind::Punctuation(Punctuator::Plus))
            || self.is_match(TokenKind::Punctuation(Punctuator::Minus))
        {
            let op = self.bump();
            let right = self.factor_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Punctuation(Punctuator::Plus) => BinaryOp::Add,
                    TokenKind::Punctuation(Punctuator::Minus) => BinaryOp::Sub,
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
        while self.is_match(TokenKind::Punctuation(Punctuator::Ge))
            || self.is_match(TokenKind::Punctuation(Punctuator::Gt))
            || self.is_match(TokenKind::Punctuation(Punctuator::Le))
            || self.is_match(TokenKind::Punctuation(Punctuator::Lt))
        {
            let op = self.bump();
            let right = self.term_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Punctuation(Punctuator::Ge) => BinaryOp::Ge,
                    TokenKind::Punctuation(Punctuator::Gt) => BinaryOp::Gt,
                    TokenKind::Punctuation(Punctuator::Le) => BinaryOp::Le,
                    TokenKind::Punctuation(Punctuator::Lt) => BinaryOp::Lt,
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
        while self.is_match(TokenKind::Punctuation(Punctuator::DoubleEq))
            || self.is_match(TokenKind::Punctuation(Punctuator::BangEq))
        {
            let op = self.bump();
            let right = self.compare_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Punctuation(Punctuator::DoubleEq) => BinaryOp::Eq,
                    TokenKind::Punctuation(Punctuator::BangEq) => BinaryOp::Ne,
                    _ => unreachable!(),
                },
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        left
    }

    /// Logical and expression parsing
    fn logical_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();
        while self.is_match(TokenKind::Punctuation(Punctuator::DoubleAmp)) {
            self.bump();
            let right = self.equality_expr();
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

    /// Logical or expression parsing
    fn logical_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_and_expr();
        while self.is_match(TokenKind::Punctuation(Punctuator::DoubleBar)) {
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
    fn is_match(&mut self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    true
                } else {
                    false
                }
            }
            None => panic!("eof"),
        }
    }

    /// Retrieves current token
    fn peek(&self) -> &Token {
        match &self.current {
            Some(tk) => tk,
            None => panic!("eof"),
        }
    }

    /// Retrieves previous token
    fn prev(&self) -> &Token {
        match &self.previous {
            Some(tk) => tk,
            None => panic!("eof"),
        }
    }

    /// Expects token with kind
    fn expect(&mut self, tk: TokenKind) {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    self.bump();
                } else {
                    panic!("missmatch: {:?} and {:?}", it.kind, tk);
                }
            }
            None => panic!("eof"),
        }
    }

    /// Advances current token
    fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
