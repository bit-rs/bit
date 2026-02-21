/// Imports
use crate::Parser;
use ast::stmt::{Block, Range, StmtKind};
use common::token::TokenKind;

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
            let end_span = self.peek().span.clone();
            Range::IncludeLast(start_span + end_span, from, to)
        } else {
            let to = self.expr();
            let end_span = self.peek().span.clone();
            Range::ExcludeLast(start_span + end_span, from, to)
        }
    }

    /// Break statement
    fn break_stmt(&mut self) -> StmtKind {
        StmtKind::Break
    }

    /// Continue statement
    fn continue_stmt(&mut self) -> StmtKind {
        StmtKind::Continue
    }

    /// Return statement
    fn return_stmt(&mut self) -> StmtKind {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Return);
        if self.check(TokenKind::Semi) {
            let end_span = self.prev().span.clone();
            StmtKind::Return(None)
        } else {
            let value = self.expr();
            let end_span = self.prev().span.clone();
            StmtKind::Return(Some(value))
        }
    }

    /// Statement kind parsing
    fn stmt_kind(&mut self) -> StmtKind {
        // Parsing statement
        let kind = match self.peek().kind {
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Id => self.assignment(),
            TokenKind::Use => self.use_stmt(),
            _ => Statement::Expr(self.expr()),
        };
        // If statement requires semicolon
        if kind.requires_semi() {
            self.expect(TokenKind::Semi);
        }
        kind
    }

    /// Satement parsing
    fn stmt(&mut self) -> StmtKind {}

    /// Block parsing
    pub fn block(&mut self) -> Block {
        let mut stmts = Vec::new();
        self.expect(TokenKind::Lbrace);
        while !self.check(TokenKind::Rbrace) {
            stmts.push(self.stmt());
        }
        self.expect(TokenKind::Rbrace);
        Block { stmts }
    }
}
