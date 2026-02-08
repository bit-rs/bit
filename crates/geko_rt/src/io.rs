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

/// Resolves use path
pub(crate) fn resolve_use_path(path: &str) -> Utf8PathBuf {
    // Todo: remove unwrap
    let mut path_buf = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap();
    path_buf.push(Utf8PathBuf::from(format!("{path}.gk")));
    path_buf
}
