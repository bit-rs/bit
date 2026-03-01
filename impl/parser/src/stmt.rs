/// Imports
use crate::{Parser, errors::ParseError};
use ast::{
    atom::Mutability,
    stmt::{Block, Range, Stmt, StmtKind},
};
use common::token::{Span, TokenKind};
use macros::bail;

/// Implementation
impl<'s> Parser<'s> {
    /// Range parsing
    fn range(&mut self) -> Range {
        let start_span = self.peek().span.clone();
        let from = self.expr();
        self.expect(TokenKind::DoubleDot);

        // If `=` given
        if self.check(TokenKind::Eq) {
            self.bump();
            let to = self.expr();
            let end_span = self.prev().span.clone();
            Range::IncludeLast(start_span + end_span, from, to)
        } else {
            let to = self.expr();
            let end_span = self.prev().span.clone();
            Range::ExcludeLast(start_span + end_span, from, to)
        }
    }

    /// Break statement
    fn break_stmt(&mut self) -> StmtKind {
        // Bumping `break`
        self.bump();
        StmtKind::Break
    }

    /// Continue statement
    fn continue_stmt(&mut self) -> StmtKind {
        // Bumping `continue`
        self.bump();
        StmtKind::Continue
    }

    /// Return statement
    fn return_stmt(&mut self) -> StmtKind {
        // Bumping `return`
        self.bump();

        if self.check(TokenKind::Semi) {
            StmtKind::Return(None)
        } else {
            let value = self.expr();
            StmtKind::Return(Some(value))
        }
    }

    /// While statement
    fn while_stmt(&mut self) -> StmtKind {
        // Bumping `while`
        self.bump();

        let expr = self.expr();
        let block = self.block();

        StmtKind::While(expr, block)
    }

    /// For statement
    fn for_stmt(&mut self) -> StmtKind {
        // Bumping `for`
        self.bump();

        let var = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::In);
        let range = self.range();
        let block = self.block();

        StmtKind::For(var, range, block)
    }

    /// Let statement
    fn let_stmt(&mut self) -> StmtKind {
        // Bumping `let`
        self.bump();

        // Checking binding mutability
        let mutability = if self.check(TokenKind::Mut) {
            self.bump();
            Mutability::Mut
        } else {
            Mutability::Immut
        };

        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Eq);
        let expr = self.expr();

        StmtKind::Let(name, mutability, expr)
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
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
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
