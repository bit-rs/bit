/// Imports
use crate::{Parser, errors::ParseError};
use ast::expr::{AssignOp, BinOp, Expr, ExprKind, Lit, UnOp};
use common::token::{Span, TokenKind};
use macros::{bail, bug};

/// Expr parsing implementation
impl<'s> Parser<'s> {
    /// Creates expr by span and kind
    fn mk_expr(&self, span: Span, kind: ExprKind) -> Expr {
        Expr { span, kind }
    }

    /// Group expression parsing
    fn group(&mut self) -> Expr {
        self.expect(TokenKind::Lparen);
        let expr = self.expr();
        self.expect(TokenKind::Rparen);
        expr
    }

    /// Variable parsing
    fn variable(&mut self) -> Expr {
        // parsing base identifier
        let start_span = self.peek().span.clone();
        let id = self.expect(TokenKind::Id).lexeme;

        // result node
        let mut result = self.mk_expr(start_span.clone(), ExprKind::Id(id));

        // checking for dots and parens
        loop {
            // checking for chain `a.b.c.d`
            if self.check(TokenKind::Dot) {
                self.bump();

                let id = self.expect(TokenKind::Id).lexeme;
                let end_span = self.prev().span.clone();

                result = self.mk_expr(
                    start_span.clone() + end_span,
                    ExprKind::Field(Box::new(result), id),
                );
                continue;
            }

            // checking for call
            if self.check(TokenKind::Lparen) {
                let args = self.sep_by(
                    TokenKind::Lparen,
                    TokenKind::Rparen,
                    TokenKind::Comma,
                    |p| p.expr(),
                );
                let end_span = self.prev().span.clone();

                result = self.mk_expr(
                    start_span.clone() + end_span,
                    ExprKind::Call(Box::new(result), args),
                );

                continue;
            }

            // breaking cycle
            break;
        }
        result
    }

    /// If expression parsing
    fn if_expr(&mut self) -> Expr {
        // Bumping `if`
        let start_span = self.peek().span.clone();
        self.bump();

        // Parsing if block
        let expr = self.expr();
        let block = {
            let block = self.block();
            self.mk_expr(block.span.clone(), ExprKind::Block(Box::new(block)))
        };

        // Parsing else block
        if self.check(TokenKind::Else) {
            self.bump();

            let branch = if self.check(TokenKind::If) {
                self.if_expr()
            } else {
                let block = self.block();
                self.mk_expr(block.span.clone(), ExprKind::Block(Box::new(block)))
            };

            let end_span = self.prev().span.clone();
            self.mk_expr(
                start_span + end_span,
                ExprKind::If(Box::new(expr), Box::new(block), Some(Box::new(branch))),
            )
        } else {
            let end_span = self.prev().span.clone();
            self.mk_expr(
                start_span + end_span,
                ExprKind::If(Box::new(expr), Box::new(block), None),
            )
        }
    }

    /// Closure expression parsing
    fn closure_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();

        // If arguments presented
        if self.check(TokenKind::Bar) {
            // Collecting params
            let params = self.sep_by(TokenKind::Bar, TokenKind::Bar, TokenKind::Comma, |p| {
                p.expect(TokenKind::Id).lexeme
            });

            let body = self.expr();
            let end_span = self.prev().span.clone();

            self.mk_expr(
                start_span + end_span,
                ExprKind::Closure(params, Box::new(body)),
            )
        } else {
            // Bumping double bar `||`
            self.bump();

            let body = self.expr();
            let end_span = self.prev().span.clone();

            self.mk_expr(
                start_span + end_span,
                ExprKind::Closure(Vec::new(), Box::new(body)),
            )
        }
    }

    /// Atom expression parsing
    fn atom(&mut self) -> Expr {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Lparen => self.group(),
            TokenKind::Number => {
                self.bump();
                self.mk_expr(tk.span, ExprKind::Lit(Lit::Number(tk.lexeme)))
            }
            TokenKind::String => {
                self.bump();
                self.mk_expr(tk.span, ExprKind::Lit(Lit::String(tk.lexeme)))
            }
            TokenKind::Bool => {
                self.bump();
                self.mk_expr(
                    tk.span,
                    ExprKind::Lit(Lit::Bool(match tk.lexeme.as_str() {
                        "true" => true,
                        "false" => false,
                        _ => bug!("non-bool value in bool literal"),
                    })),
                )
            }
            TokenKind::Id => self.variable(),
            TokenKind::If => self.if_expr(),
            TokenKind::Bar | TokenKind::DoubleBar => self.closure_expr(),
            _ => bail!(ParseError::UnexpectedExprToken {
                got: tk.kind,
                src: self.source.clone(),
                span: tk.span.1.into(),
            }),
        }
    }

    /// Unary expression parsing
    fn unary_expr(&mut self) -> Expr {
        if self.check(TokenKind::Minus)
            || self.check(TokenKind::Bang)
            || self.check(TokenKind::Ampersand)
            || self.check(TokenKind::Star)
        {
            let start_span = self.peek().span.clone();

            let op = match self.bump().kind {
                TokenKind::Minus => UnOp::Neg,
                TokenKind::Bang => UnOp::Bang,
                TokenKind::Ampersand => {
                    if self.check(TokenKind::Mut) {
                        self.bump();
                        UnOp::MutRef
                    } else {
                        UnOp::Deref
                    }
                }
                TokenKind::Star => UnOp::Deref,
                _ => unreachable!(),
            };

            let value = self.unary_expr();
            let end_span = self.prev().span.clone();

            return self.mk_expr(start_span + end_span, ExprKind::Unary(op, Box::new(value)));
        }

        self.atom()
    }

    /// Factor expression parsing
    fn factor_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.unary_expr();

        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let op = match self.bump().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Mod,
                _ => unreachable!(),
            };

            let right = self.unary_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(op, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// Term expression parsing
    fn term_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.factor_expr();

        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let op = match self.bump().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };

            let right = self.factor_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(op, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// Compare expression parsing
    fn compare_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.term_expr();

        while self.check(TokenKind::Ge)
            || self.check(TokenKind::Gt)
            || self.check(TokenKind::Le)
            || self.check(TokenKind::Lt)
        {
            let op = match self.bump().kind {
                TokenKind::Ge => BinOp::Ge,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Lt => BinOp::Lt,
                _ => unreachable!(),
            };

            let right = self.factor_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(op, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// Equality expression parsing
    fn equality_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.compare_expr();

        while self.check(TokenKind::DoubleEq) || self.check(TokenKind::BangEq) {
            let op = match self.bump().kind {
                TokenKind::DoubleEq => BinOp::Eq,
                TokenKind::BangEq => BinOp::Ne,
                _ => unreachable!(),
            };

            let right = self.compare_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(op, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Bitwise and` expression parsing
    fn bitwise_and_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();

        while self.check(TokenKind::Ampersand) {
            self.bump();

            let right = self.equality_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(BinOp::BitAnd, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Bitwise xor` expression parsing
    fn bitwise_xor_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_and_expr();

        while self.check(TokenKind::Caret) {
            self.bump();

            let right = self.bitwise_and_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(BinOp::Xor, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Bitwise or` expression parsing
    fn bitwise_or_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_xor_expr();

        while self.check(TokenKind::Bar) {
            self.bump();

            let right = self.bitwise_xor_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(BinOp::BitOr, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Logical and` expression parsing
    fn logical_and_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_or_expr();

        while self.check(TokenKind::DoubleAmp) {
            self.bump();

            let right = self.bitwise_or_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(BinOp::And, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Logical or` expression parsing
    fn logical_or_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_and_expr();

        while self.check(TokenKind::DoubleBar) {
            self.bump();

            let right = self.logical_and_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Bin(BinOp::Or, Box::new(left), Box::new(right)),
            )
        }

        left
    }

    /// `Assign` expression parsing
    fn assign_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_or_expr();

        while self.check(TokenKind::Eq)
            | self.check(TokenKind::AmpersandEq)
            | self.check(TokenKind::BarEq)
            | self.check(TokenKind::PlusEq)
            | self.check(TokenKind::MinusEq)
            | self.check(TokenKind::StarEq)
            | self.check(TokenKind::SlashEq)
            | self.check(TokenKind::PercentEq)
            | self.check(TokenKind::CaretEq)
        {
            let op = match self.bump().kind {
                TokenKind::Eq => AssignOp::Eq,
                TokenKind::AmpersandEq => AssignOp::AndEq,
                TokenKind::BarEq => AssignOp::OrEq,
                TokenKind::PlusEq => AssignOp::AddEq,
                TokenKind::MinusEq => AssignOp::SubEq,
                TokenKind::StarEq => AssignOp::MulEq,
                TokenKind::SlashEq => AssignOp::DivEq,
                TokenKind::PercentEq => AssignOp::ModEq,
                TokenKind::CaretEq => AssignOp::XorEq,
                _ => unreachable!(),
            };

            let right = self.logical_or_expr();
            let end_span = self.prev().span.clone();

            left = self.mk_expr(
                start_span.clone() + end_span,
                ExprKind::Assign(Box::new(left), op, Box::new(right)),
            )
        }

        left
    }
    /// Parses expression
    pub fn expr(&mut self) -> Expr {
        self.assign_expr()
    }
}
