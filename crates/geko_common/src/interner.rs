/// Imports
use camino::Utf8PathBuf;
use id_arena::Arena;

/// `Interner` is a structure responsible
/// for storing and uniquely identifying file paths.
pub struct Interner {
    /// Files arena
    files: Arena<Utf8PathBuf>,
}
