/// Imports
use crate::{Parser, errors::ParseError};
use ast::stmt::{Block, Stmt, StmtKind};
use common::token::{Span, TokenKind};
use macros::bail;

/// Implementation
impl<'s> Parser<'s> {
    /// Let statement
    fn let_stmt(&mut self) -> StmtKind {
        // Bumping `let`
        self.bump();

        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Eq);
        let expr = self.expr();

        StmtKind::Let(name, expr)
    }

    /// Expression statement
    fn expr_stmt(&mut self) -> StmtKind {
        let expr = self.expr();
        if self.check(TokenKind::Semi) {
            StmtKind::Semi(expr)
        } else {
            StmtKind::Expr(expr)
        }
    }

    /// Statement kind parsing
    fn stmt_kind(&mut self) -> StmtKind {
        // Parsing statement
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Let => self.let_stmt(),
            _ => self.expr_stmt(),
        }
    }

    /// Statement parsing with semicolon
    fn stmt_inner(&mut self) -> (Span, StmtKind) {
        // Parsing statement kind
        let start_span = self.peek().span.clone();
        let kind = self.stmt_kind();

        // If semicolon presented
        if self.check(TokenKind::Semi) {
            self.bump();
            let end_span = self.prev().span.clone();
            (start_span + end_span, kind)
        }
        // If not
        else {
            let end_span = self.prev().span.clone();

            // If statement doesn't requires semicolon or the block starts
            if !kind.requires_semi() || self.check(TokenKind::Lbrace) {
                (start_span + end_span, kind)
            } else {
                bail!(ParseError::ExpectedSemicolon {
                    src: self.source.clone(),
                    span: (start_span + end_span).1.into()
                })
            }
        }
    }

    /// Statement parsing
    fn stmt(&mut self) -> Stmt {
        let (span, kind) = self.stmt_inner();

        Stmt { span, kind }
    }

    /// Block parsing
    pub fn block(&mut self) -> Block {
        let start_span = self.peek().span.clone();
        let mut stmts = Vec::new();

        self.expect(TokenKind::Lbrace);
        while !self.check(TokenKind::Rbrace) {
            stmts.push(self.stmt());
        }
        self.expect(TokenKind::Rbrace);
        let end_span = self.prev().span.clone();

        Block {
            span: start_span + end_span,
            stmts,
        }
    }
}
