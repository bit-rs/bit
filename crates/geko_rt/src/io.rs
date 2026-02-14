/// Imports
use camino::Utf8PathBuf;
use geko_common::bail;
use miette::Diagnostic;
use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
};
use thiserror::Error;

/// IO error
#[derive(Error, Diagnostic, Debug)]
pub enum IOError {
    /// File not found
    #[error("file `{0}` not found")]
    #[diagnostic(code(io::file_not_found))]
    FileNotFound(Utf8PathBuf),
    /// Non-utf8 path
    #[error("invalid utf-8 path `{0}`")]
    #[diagnostic(code(io::non_utf8_path))]
    NonUtf8Path(PathBuf),
    /// Cwd not available
    #[error("failed to get current working directory due io error: {0}")]
    #[diagnostic(code(io::cwd_not_available))]
    CwdNotAvailable(io::Error),
}

/// Reads file
pub(crate) fn read(path: &Utf8PathBuf) -> String {
    // Reading module
    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) => bail!(IOError::FileNotFound(path.clone())),
    }
}

/// Resolves use path.
///
/// Returns `None` if path isn't exists or fs isn't available.
/// Otherwise returns `Some(Utf8PathBuf)`
///
pub(crate) fn resolve_use_path(path: &str) -> Option<Utf8PathBuf> {
    // Retrieving current directory
    match std::env::current_dir() {
        // Note: from_path_buf with reference is not implemented.
        Ok(cwd) => match Utf8PathBuf::from_path_buf(cwd.clone()) {
            Ok(mut dir) => {
                // Appending path to cwd
                dir.push(Utf8PathBuf::from(format!("{path}.gk")));
                // If path exists
                if dir.exists() {
                    Some(dir)
                } // If not
                else {
                    None
                }
            }
            Err(_) => bail!(IOError::NonUtf8Path(cwd)),
        },
        Err(err) => match err.kind() {
            // If fs operations are not supported (wasm)
            ErrorKind::Unsupported => None,
            // Reporting other errors
            _ => bail!(IOError::CwdNotAvailable(err)),
        },
    }
}
