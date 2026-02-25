/// Imports
use crate::{Parser, errors::ParseError};
use bit_ast::{expr::Expr, stmt::Stmt};
use bit_lex::tokens::TokenKind;

/// Implementation
impl<'s> Parser<'s> {
    /// Let statement
    fn let_stmt(&mut self) -> Stmt {
        // Bumping `let`
        let start_span = self.peek().span.clone();
        self.bump();
        let name = self.expect(TokenKind::Id).lexeme;

        // Parsing hint
        let hint = if self.check(TokenKind::Colon) {
            Some(self.type_hint())
        } else {
            None
        };
        self.expect(TokenKind::Eq);
        let expr = self.expr();

        let end_span = self.prev().span.clone();

        Stmt::Let(start_span + end_span, name, hint, expr)
    }

    /// Expression statement
    fn expr_stmt(&mut self) -> Stmt {
        let expr = self.expr();
        if self.check(TokenKind::Semi) {
            Stmt::Semi(expr)
        } else {
            Stmt::Expr(expr)
        }
    }

    /// Statement parsing with semicolon
    fn stmt(&mut self) -> Stmt {
        // Parsing statement kind
        match self.peek().kind {
            TokenKind::Let => self.let_stmt(),
            _ => self.expr_stmt(),
        }
    }

    /// Block parsing
    pub fn block(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let stmts = self.sep_by_2(TokenKind::Semi, |p| p.stmt());
        let end_span = self.prev().span.clone();

        Expr::Block(start_span + end_span, stmts)
    }
}
