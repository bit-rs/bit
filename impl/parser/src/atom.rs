/// Imports
use crate::Parser;
use ast::atom::{Param, TypeHint};
use common::token::TokenKind;

/// Implementation
impl<'s> Parser<'s> {
    /// Parses generic args
    fn generic_args(&mut self) -> Vec<TypeHint> {
        self.sep_by(TokenKind::Lt, TokenKind::Gt, TokenKind::Comma, |p| {
            p.type_hint()
        })
    }

    /// Parses generic params
    pub fn generic_params(&mut self) -> Vec<String> {
        if self.check(TokenKind::Lt) {
            self.sep_by(TokenKind::Lt, TokenKind::Gt, TokenKind::Comma, |p| {
                p.expect(TokenKind::Id).lexeme
            })
        } else {
            Vec::new()
        }
    }

    /// Parses params
    pub fn params(&mut self) -> Vec<Param> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |p| {
                let start_span = p.peek().span.clone();

                let name = p.expect(TokenKind::Id).lexeme;
                p.expect(TokenKind::Colon);
                let hint = p.type_hint();

                let end_span = p.prev().span.clone();

                Param {
                    span: start_span + end_span,
                    name,
                    hint,
                }
            },
        )
    }

    /// Ref type hint
    fn ref_type_hint(&mut self) -> TypeHint {
        // bumping `&`
        let start_span = self.peek().span.clone();
        self.bump();

        // if `mut` ref
        if self.check(TokenKind::Mut) {
            self.bump();

            let hint = Box::new(self.type_hint());
            let end_span = self.prev().span.clone();

            TypeHint::MutRef(start_span + end_span, hint)
        } else {
            let hint = Box::new(self.type_hint());
            let end_span = self.prev().span.clone();

            TypeHint::Ref(start_span + end_span, hint)
        }
    }

    /// Parses id type hint
    fn id_type_hint(&mut self) -> TypeHint {
        // bumping id
        let start_span = self.peek().span.clone();
        let id = self.bump().lexeme;

        // if dot presented
        if self.check(TokenKind::Dot) {
            self.bump();

            let name = self.expect(TokenKind::Id).lexeme;
            let generics = self.generic_args();
            let end_span = self.prev().span.clone();

            TypeHint::Module {
                span: start_span + end_span,
                module: id,
                name: name,
                generics,
            }
        }
        // If not
        else {
            let generics = self.generic_args();
            let end_span = self.prev().span.clone();

            TypeHint::Local {
                span: start_span + end_span,
                name: id,
                generics,
            }
        }
    }

    /// Parses function type hint
    fn fn_type_hint(&mut self) -> TypeHint {
        // bumping `fn`
        let start_span = self.peek().span.clone();
        self.bump();

        // parsing params
        let params = self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |p| p.type_hint(),
        );

        // parsing return type
        let ret = if self.check(TokenKind::Arrow) {
            self.bump();
            Box::new(self.type_hint())
        } else {
            Box::new(self.type_hint())
        };
        let end_span = self.prev().span.clone();

        TypeHint::Function {
            span: start_span + end_span,
            params,
            ret,
        }
    }

    /// Parses type hint
    pub fn type_hint(&mut self) -> TypeHint {
        if self.check(TokenKind::Ampersand) {
            self.ref_type_hint()
        } else if self.check(TokenKind::Fn) {
            self.fn_type_hint()
        } else {
            self.id_type_hint()
        }
    }
}
