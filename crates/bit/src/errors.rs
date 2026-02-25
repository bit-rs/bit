/// Imports
use miette::Diagnostic;
use std::{io::ErrorKind, path::PathBuf};
use thiserror::Error;

/// Cli error
#[derive(Debug, Error, Diagnostic)]
pub enum CliError {
    #[error("failed to retrieve cwd.")]
    #[diagnostic(
        code(pkg::failed_to_retrieve_cwd),
        help("check existence of current working directory.")
    )]
    FailedToRetrieveCwd,
    #[error("{path} already exists.")]
    #[diagnostic(code(pkg::already_exists))]
    AlreadyExists { path: String },
    #[error("failed to create directory {path}: {kind}.")]
    #[diagnostic(code(pkg::failed_to_create_dir))]
    FailedToCreateDir { path: String, kind: ErrorKind },
    #[error("failed to convert path {path} to utf8 path.")]
    #[diagnostic(code(pkg::wrong_utf8_path))]
    WrongUtf8Path { path: PathBuf },
    #[error("runtime {rt} is invalid.")]
    #[diagnostic(code(pkg::invalid_runtime))]
    InvalidRuntime { rt: String },
}
