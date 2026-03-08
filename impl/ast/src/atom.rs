/// Imports
use common::token::Span;

/// Represents item publicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Publicity {
    Pub,
    Private,
}

/// Represents type hint
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeHint {
    /// Local type
    Local {
        span: Span,
        name: String,
        args: Vec<TypeHint>,
    },
    /// Module type
    Module {
        span: Span,
        module: String,
        name: String,
        args: Vec<TypeHint>,
    },
    /// Function type
    Function {
        span: Span,
        params: Vec<TypeHint>,
        ret: Box<TypeHint>,
    },
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
