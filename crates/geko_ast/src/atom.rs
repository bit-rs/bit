use crate::stmt::Block;

/// Binary operator
pub enum BinaryOp {
    Add, //  +
    Sub, //  -
    Mul, //  *
    Div, //  /
    Mod, //  %
    And, //  &&
    Or,  //  ||
    Gt,  //  >
    Ge,  //  >=
    Lt,  //  <
    Le,  //  <=
    Not, //  !
    Eq,  //  ==
    Ne,  //  !=
}

/// Unary operator
pub enum UnaryOp {
    Neg,  // -
    Bang, // !
}

/// Literal
pub enum Lit {
    /// Number literal
    Number(String),
    /// String literal
    String(String),
    /// Bool literal
    Bool(String),
}

/// Function
pub struct Function {
    // Function name
    pub name: String,
    // Function params
    pub params: Vec<String>,
    // Block
    pub block: Block,
}
