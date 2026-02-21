/// Imports
use common::token::Span;

/// Represents item publicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Publicity {
    Pub,
    Private,
}

/// Represents declaration mutability
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mutability {
    Mut,
    Immut,
}

/// Represents type hint
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeHint {
    /// Local type
    Local {
        span: Span,
        name: String,
        generics: Vec<TypeHint>,
    },
    /// Module type
    Module {
        span: Span,
        module: String,
        name: String,
        generics: Vec<TypeHint>,
    },
    /// Function type
    Function {
        span: Span,
        params: Vec<TypeHint>,
        ret: Box<TypeHint>,
    },
    /// Ref type
    Ref(Span, Box<TypeHint>),
    /// Mut ref type
    MutRef(Span, Box<TypeHint>),
    /// Unit type
    Unit(Span),
    /// Not known
    Infer,
}

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub hint: TypeHint,
}
