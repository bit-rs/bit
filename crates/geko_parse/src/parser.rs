use std::sync::Arc;

/// Import
use geko_ast::{
    atom::{AssignOp, BinaryOp, Function, Lit, UnaryOp},
    expr::{Expression, Range},
    stmt::{Block, Statement},
};
use geko_common::bail;
use geko_lex::{
    lexer::Lexer,
    token::{Token, TokenKind},
};
use miette::NamedSource;

use crate::errors::ParseError;

/// Parser converts a stream of tokens
/// produced by the lexer into an abstract syntax tree (AST).
pub struct Parser<'s> {
    /// Named source of the file
    source: Arc<NamedSource<String>>,

    /// Lexer used to iterate over tokens
    lexer: Lexer<'s>,

    /// Previously consumed token
    /// (useful for spans and error reporting)
    previous: Option<Token>,

    /// Current token under inspection
    current: Option<Token>,

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
    pub fn parse(&mut self) -> Block {
        let mut statements = Vec::new();
        while self.current.is_some() {
            statements.push(self.stmt())
        }
        Block { statements }
    }

    /// Range parsing
    fn range(&mut self) -> Range {
        let start_span = self.peek().span.clone();
        let from = self.expr();
        self.expect(TokenKind::DoubleDot);

        // If `=` given
        if self.check(TokenKind::Eq) {
            self.bump();
            let to = self.expr();
            let end_span = self.peek().span.clone();
            Range::IncludeLast {
                span: start_span + end_span,
                from,
                to,
            }
        } else {
            let to = self.expr();
            let end_span = self.peek().span.clone();
            Range::ExcludeLast {
                span: start_span + end_span,
                from,
                to,
            }
        }
    }

    /// For statement parsing
    fn for_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::For);
        let var = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::In);
        let range = self.range();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Statement::For {
            span: start_span + end_span,
            var,
            range,
            block,
        }
    }

    /// While statement parsing
    fn while_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::While);
        let condition = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Statement::While {
            span: start_span + end_span,
            condition,
            block,
        }
    }

    /// Else branch
    fn else_branch(&mut self) -> Statement {
        self.expect(TokenKind::Else);
        if self.check(TokenKind::If) {
            self.if_stmt()
        } else {
            Statement::Block(Box::new(self.block()))
        }
    }

    /// If statement parsing
    fn if_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::If);
        let condition = self.expr();
        let then = self.block();
        let else_ = if self.check(TokenKind::Else) {
            Some(Box::new(self.else_branch()))
        } else {
            None
        };

        let end_span = self.prev().span.clone();

        Statement::If {
            span: start_span + end_span,
            condition,
            then,
            else_,
        }
    }

    /// Let statement parsing
    fn let_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::Let);
        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Eq);
        let value = self.expr();

        let end_span = self.prev().span.clone();

        Statement::Let {
            span: start_span + end_span,
            name,
            value,
        }
    }

    /// Type declaration parsing
    fn type_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Type);
        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Lbrace);
        let mut methods = Vec::new();
        while !self.check(TokenKind::Rbrace) {
            methods.push(self.function())
        }
        self.expect(TokenKind::Rbrace);
        let end_span = self.prev().span.clone();

        Statement::Type {
            span: start_span + end_span,
            name,
            methods,
        }
    }

    /// Assignment statement
    fn assignment_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        let variable = self.variable();
        // Checking for ssignment operator
        let op = match self.peek().kind {
            TokenKind::PlusEq => Some(AssignOp::Add),
            TokenKind::MinusEq => Some(AssignOp::Sub),
            TokenKind::StarEq => Some(AssignOp::Mul),
            TokenKind::SlashEq => Some(AssignOp::Div),
            TokenKind::PercentEq => Some(AssignOp::Mod),
            TokenKind::AmpersandEq => Some(AssignOp::BitAnd),
            TokenKind::BarEq => Some(AssignOp::BitOr),
            TokenKind::CaretEq => Some(AssignOp::Xor),
            TokenKind::Eq => Some(AssignOp::Assign),
            _ => None,
        };
        // Checking assignment operator existence
        match op {
            // If operator found
            Some(op) => {
                // Bumping operator
                self.bump();
                let value = self.expr();
                let end_span = self.prev().span.clone();
                match variable {
                    Expression::Variable { name, .. } => Statement::Assign {
                        span: start_span + end_span,
                        name,
                        op,
                        value,
                    },
                    Expression::Field {
                        name, container, ..
                    } => Statement::Set {
                        span: start_span + end_span,
                        container: *container,
                        name,
                        op,
                        value,
                    },
                    _ => bail!(ParseError::InvalidUseOfAssignOp {
                        src: self.source.clone(),
                        first_span: (start_span + end_span).1.into()
                    }),
                }
            }
            // Else
            None => Statement::Expr(variable),
        }
    }

    /// Break statement
    fn break_stmt(&mut self) -> Statement {
        let span = self.expect(TokenKind::Break).span;
        Statement::Break(span)
    }

    /// Continue statement
    fn continue_stmt(&mut self) -> Statement {
        let span = self.expect(TokenKind::Continue).span;
        Statement::Continue(span)
    }

    /// Return statement
    fn return_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Return);
        if self.check(TokenKind::Semi) {
            let end_span = self.prev().span.clone();
            Statement::Return {
                span: start_span + end_span,
                expr: None,
            }
        } else {
            let value = self.expr();
            let end_span = self.prev().span.clone();
            Statement::Return {
                span: start_span + end_span,
                expr: Some(value),
            }
        }
    }

    /// Satement parsing
    fn stmt(&mut self) -> Statement {
        // Parsing statement
        let stmt = match self.peek().kind {
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Type => self.type_stmt(),
            TokenKind::Fn => Statement::Function(self.function()),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Id => self.assignment_stmt(),
            TokenKind::Use => todo!(),
            _ => Statement::Expr(self.expr()),
        };
        // If statement requires semicolon
        if stmt.requires_semi() {
            self.expect(TokenKind::Semi);
        }
        stmt
    }

    /// Block parsing
    fn block(&mut self) -> Block {
        let mut statements = Vec::new();
        self.expect(TokenKind::Lbrace);
        while !self.check(TokenKind::Rbrace) {
            statements.push(self.stmt());
        }
        self.expect(TokenKind::Rbrace);
        Block { statements }
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

    /// Arguments parsing
    pub(crate) fn args(&mut self) -> Vec<Expression> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expr(),
        )
    }

    /// Single parameter parsing
    fn param(&mut self) -> String {
        return self.expect(TokenKind::Id).lexeme;
    }

    /// Parameters parsing
    pub(crate) fn params(&mut self) -> Vec<String> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.param(),
        )
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
        while self.check(TokenKind::Star) || self.check(TokenKind::Slash) {
            let op = self.bump();
            let right = self.unary_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Star => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
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

    /// Logical and expression parsing
    fn logical_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();
        while self.check(TokenKind::DoubleAmp) {
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
    fn check(&self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    true
                } else {
                    false
                }
            }
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Retrieves current token
    fn peek(&self) -> &Token {
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
    fn prev(&self) -> &Token {
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
    fn expect(&mut self, tk: TokenKind) -> Token {
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
    fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
