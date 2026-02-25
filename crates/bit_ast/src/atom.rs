/// Imports
use bit_common::span::Span;
use ecow::EcoString;

/// Represents item publicity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Publicity {
    Pub,
    Priv,
}

/// Binary operator
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,    // `+`
    Sub,    // `-`
    Mul,    // `*`
    Div,    // `/`
    Mod,    // `%`
    Eq,     // `==`
    NotEq,  // `!=`
    Gt,     // `>`
    Ge,     // `>=`
    Lt,     // `<`
    Le,     // `<=`
    And,    // `&&`
    Or,     // `||`
    Xor,    // `^`
    BitAnd, // `&`
    BitOr,  // `|`
    Concat, // `<>`
}

/// Unary operator
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg,  // -
    Bang, // !
}

/// Represents type hint
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeHint {
    /// Local type
    Local {
        span: Span,
        name: EcoString,
        generics: Vec<TypeHint>,
    },
    /// Module type
    Module {
        span: Span,
        module: EcoString,
        name: EcoString,
        generics: Vec<TypeHint>,
    },
    /// Function type
    Function {
        span: Span,
        params: Vec<TypeHint>,
        ret: Box<TypeHint>,
    },
    /// Unit type
    Unit(Span),
}

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: EcoString,
    pub hint: TypeHint,
}
