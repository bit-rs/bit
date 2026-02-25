/// Imports
use ecow::EcoString;

/// Represents a stack-based context for managing generic type parameters
/// during type inference and hydration.
///
/// The `Generics` structure maintains a stack of generic scopes.
/// Each scope (a `Vec<String>`) contains the names of generic type.
///
/// # Fields
///
/// - `stack: Vec<Vec<String>>` — A stack of scopes,
///   where each scope holds the names of generics currently active within that scope.
///
/// # Example
///
/// ```watt
/// let mut generics = Generics::new();
/// generics.push_scope(vec!["T".into(), "U".into()]);
/// generics.push_scope(vec!["V".into()]);
///
/// // Top of stack: ["V"]
/// // Second scope: ["T", "U"]
///
/// generics.pop_scope(); // exits scope with ["V"]
/// ```
///
/// # Notes
/// - contains method checks generic name
///   only in the last scope.
///
#[derive(Default, Debug)]
pub struct Generics {
    stack: Vec<Vec<EcoString>>,
}

/// Implementation
impl Generics {
    /// Pushes the scope onto the stack
    /// and inserts given generic arguments
    /// in it.
    ///
    /// # Parameters
    /// - `generics: Vec<EcoString>`
    ///   Generic parameter names.
    ///
    pub fn push_scope(&mut self, generics: Vec<EcoString>) {
        self.stack.push(generics);
    }

    /// Pops scope from the stack
    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    /// Returns generic index by the name
    /// from the last scope, if generic exists.
    ///
    /// # Parameters
    /// - `name: &str`
    ///   Name of the generic
    ///
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.stack
            .last()
            .and_then(|s| s.iter().position(|p| p == name))
    }
}
