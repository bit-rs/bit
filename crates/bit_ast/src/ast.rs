/// Imports
use crate::item::{Import, Item};
use miette::NamedSource;
use std::sync::Arc;

/// Ast tree
#[derive(Debug)]
pub struct Module {
    pub source: Arc<NamedSource<String>>,
    pub imports: Vec<Import>,
    pub items: Vec<Item>,
}
