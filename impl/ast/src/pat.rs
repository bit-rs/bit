/// Assignment pattern
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pat {
    /// Deref
    Deref(Box<Pat>),

    /// Id
    Id(String),

    /// Field
    Field(Box<Pat>),
}
