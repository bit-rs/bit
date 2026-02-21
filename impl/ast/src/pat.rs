/// Assignment pattern
pub enum Pat {
    /// Deref
    Deref(Box<Pat>),

    /// Id
    Id(String),

    /// Field
    Field(Box<Pat>),
}
