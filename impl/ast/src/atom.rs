/// Imports
use common::token::Span;

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
    /// Ref type
    Ref(Box<TypeHint>),
    /// Mut ref type
    MutRef(Box<TypeHint>),
    /// Unit type
    Unit,
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
