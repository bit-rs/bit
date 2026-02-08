/// Imports
use camino::Utf8PathBuf;
use geko_common::bail;
use miette::Diagnostic;
use std::fs;
use thiserror::Error;

/// IO error
#[derive(Error, Diagnostic, Debug)]
pub enum IOError {
    /// File not found
    #[error("file `{0}` not found.")]
    #[diagnostic(code(io::file_not_found))]
    FileNotFound(Utf8PathBuf),
}

/// Reads file
pub(crate) fn read(path: &Utf8PathBuf) -> String {
    // Reading module
    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) => bail!(IOError::FileNotFound(path.clone())),
    }
}
