/// Imports
use bit_common::span::Span;

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
    Ne,     // `!=`
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

/// Assignment operation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignOp {
    AddEq, // +=
    SubEq, // -=
    MulEq, // *=
    DivEq, // /=
    ModEq, // %=
    AndEq, // &=
    OrEq,  // |=
    XorEq, // ^=
    Eq,    // =
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
    /// Unit type
    Unit(Span),
}

/// Function param
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Span,
    pub name: String,
    pub hint: TypeHint,
}
