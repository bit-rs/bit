/// Imports
use crate::{
    atom::{Param, Publicity, TypeHint},
    expr::Expr,
};
use bit_common::span::Span;
use ecow::EcoString;

/// Represents enum varisnt
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub location: Span,
    pub name: EcoString,
    pub params: Vec<TypeHint>,
}

/// Import path (e.g `this/is/some/module`)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ImportPath {
    pub address: Span,
    pub module: EcoString,
}

/// Represents import kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportKind {
    /// Represents import of module as given name
    AsName(EcoString),
    /// Represents import of module contents separated by comma
    ForNames(Vec<EcoString>),
    /// Just import of module
    Just,
}

/// Represents import declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Import {
    pub location: Span,
    pub path: ImportPath,
    pub kind: ImportKind,
}

/// Represents struct field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub location: Span,
    pub name: EcoString,
    pub himt: TypeHint,
}

/// Struct item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    location: Span,
    name: EcoString,
    publicity: Publicity,
    generics: Vec<EcoString>,
    fields: Vec<Field>,
}

/// Enum item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    location: Span,
    name: EcoString,
    publicity: Publicity,
    generics: Vec<EcoString>,
    variants: Vec<Variant>,
}

/// Function item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fn {
    location: Span,
    publicity: Publicity,
    name: EcoString,
    generics: Vec<EcoString>,
    params: Vec<Param>,
    ret: Option<TypeHint>,
    body: Expr,
}

/// Extern function item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternFn {
    location: Span,
    name: EcoString,
    publicity: Publicity,
    generics: Vec<EcoString>,
    params: Vec<Param>,
    ret: Option<TypeHint>,
    body: EcoString,
}

/// Constant item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Const {
    pub location: Span,
    pub publicity: Publicity,
    pub name: EcoString,
    pub value: Expr,
    pub himt: TypeHint,
}

/// Item declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Struct(Struct),
    Enum(Enum),
    Fn(Fn),
    ExternFn(ExternFn),
    Const(Const),
}
