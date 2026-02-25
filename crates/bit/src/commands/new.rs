/// Imports
use crate::errors::CliError;
use bit_common::bail;
use bit_pm::{config::PackageType, generate};
use std::io::ErrorKind;

/// Executes command
pub fn execute(path: String, pkg_ty: Option<PackageType>) {
    match std::fs::create_dir(&path) {
        Ok(_) => {
            let pkg_ty = pkg_ty.unwrap_or(PackageType::App);
            generate::gen_project(path.into(), pkg_ty);
        }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => bail!(CliError::AlreadyExists { path }),
            kind => bail!(CliError::FailedToCreateDir { path, kind }),
        },
    };
}
