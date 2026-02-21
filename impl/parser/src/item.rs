/// Imports
use crate::{Parser, errors::ParseError};
use ast::{
    atom::{Publicity, TypeHint},
    item::{Enum, Field, Function, Item, ItemKind, Struct, Use, UseKind, Variant},
};
use common::token::TokenKind;
use macros::bail;

/// Item parsing implementation
impl<'s> Parser<'s> {
    // Parses struct field
    fn struct_field(&mut self) -> Field {
        let start_span = self.peek().span.clone();
        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Colon);
        let hint = self.type_hint();
        let end_span = self.peek().span.clone();

        Field {
            span: start_span + end_span,
            name,
            hint,
        }
    }

    // Parses struct
    fn struct_item_kind(&mut self) -> ItemKind {
        // Bumping `struct`
        self.bump();

        // Parsing signature
        let name = self.expect(TokenKind::Id).lexeme;
        let generics = self.generic_params();

        // Parsing fields
        let fields = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.struct_field(),
        );

        ItemKind::Struct(Struct {
            name,
            generics,
            fields,
        })
    }

    // Parses enum variant
    fn enum_variant(&mut self) -> Variant {
        let start_span = self.peek().span.clone();
        let name = self.expect(TokenKind::Id).lexeme;
        let params = if self.check(TokenKind::Lparen) {
            self.sep_by(
                TokenKind::Lparen,
                TokenKind::Rparen,
                TokenKind::Comma,
                |p| p.type_hint(),
            )
        } else {
            Vec::new()
        };
        let end_span = self.peek().span.clone();

        Variant {
            span: start_span + end_span,
            name,
            params,
        }
    }

    // Parses enum
    fn enum_item_kind(&mut self) -> ItemKind {
        // Bumping `enum`
        self.bump();

        // Parsing signature
        let name = self.expect(TokenKind::Id).lexeme;
        let generics = self.generic_params();

        // Parsing variants
        let variants = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.enum_variant(),
        );

        ItemKind::Enum(Enum {
            name,
            generics,
            variants,
        })
    }

    // Parses function
    fn fn_item_kind(&mut self) -> ItemKind {
        // Bumping `fn`
        self.bump();

        // Parsing signature
        let name = self.expect(TokenKind::Id).lexeme;
        let generics = self.generic_params();
        let params = self.params();
        let ret = if self.check(TokenKind::Arrow) {
            self.bump();
            self.type_hint()
        } else {
            TypeHint::Infer
        };

        // Parsing body
        let block = self.block();

        ItemKind::Function(Function {
            name,
            generics,
            params,
            ret,
            block,
        })
    }

    /// Use path parsing
    fn use_path(&mut self) -> String {
        // Module name string
        let mut module = String::new();

        // First id
        module.push_str(&self.expect(TokenKind::Id).lexeme.clone());

        while self.check(TokenKind::Slash) {
            self.expect(TokenKind::Slash);
            module.push('/');
            module.push_str(&self.expect(TokenKind::Id).lexeme.clone());
        }

        module
    }

    // Parses use
    fn use_item_kind(&mut self) -> ItemKind {
        // Bumping `use`
        self.bump();

        // Use path
        let path = self.use_path();

        // Suffix
        let kind = if self.check(TokenKind::As) {
            self.bump();
            let name = self.expect(TokenKind::Id).lexeme;

            UseKind::As(name)
        } else if self.check(TokenKind::For) {
            self.bump();
            let names = self.sep_by_2(TokenKind::Comma, |p| p.expect(TokenKind::Id).lexeme);

            UseKind::For(names)
        } else {
            UseKind::Just
        };

        ItemKind::Use(Use { path, kind })
    }

    // Parses top-level item itself
    fn item_kind(&mut self) -> ItemKind {
        let tk = self.peek().clone();

        match &tk.kind {
            TokenKind::Struct => self.struct_item_kind(),
            TokenKind::Enum => self.enum_item_kind(),
            TokenKind::Fn => self.fn_item_kind(),
            TokenKind::Use => self.use_item_kind(),
            _ => bail!(ParseError::UnexpectedItemToken {
                got: tk.kind,
                src: self.source.clone(),
                span: tk.span.1.into(),
            }),
        }
    }

    // Parses top-level item with publicity
    fn item_inner(&mut self) -> (Publicity, ItemKind) {
        if self.check(TokenKind::Pub) {
            (Publicity::Pub, self.item_kind())
        } else {
            (Publicity::Private, self.item_kind())
        }
    }

    // Parses top-level item
    pub fn item(&mut self) -> Item {
        let start_span = self.peek().span.clone();
        let (publicity, kind) = self.item_inner();
        let end_span = self.prev().span.clone();

        Item {
            span: start_span + end_span,
            publicity,
            kind,
        }
    }
}
