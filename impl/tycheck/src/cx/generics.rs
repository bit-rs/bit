/// A scoped stack of generic parameter lists used during
/// type inference.
///
/// Each entry on the stack corresponds to one set of generic parameters — for
/// example, those declared on a function or a struct. Parameters are stored in
/// declaration order, so the index of a name in its `Vec<String>` is the same
/// integer used in `Ty::Generic(i)`.
///
/// Scopes must be pushed before entering a generic item and popped upon exit,
/// keeping the stack in sync with the AST traversal.
///
#[derive(Default, Debug)]
pub struct GenericsCx {
    stack: Vec<Vec<String>>,
}

/// Implementation
impl GenericsCx {
    /// Pushes a new generic parameter scope onto the stack.
    ///
    /// `generics` must list parameter names in declaration order so that
    /// index `i` corresponds to `Ty::Generic(i)`.
    pub fn push(&mut self, generics: Vec<String>) {
        self.stack.push(generics);
    }

    /// Removes the innermost generic parameter scope from the stack.
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    /// Looks up `name` in the innermost scope and returns its index, or
    /// `None` if the name is not present or the stack is empty.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.stack.last()?.iter().position(|g| g == name)
    }

    /// Returns the name of the generic parameter at position `idx` in the
    /// innermost scope, or `None` if the index is out of range or the stack
    /// is empty.
    pub fn name_of(&self, idx: usize) -> Option<String> {
        self.stack.last()?.get(idx).cloned()
    }
}
