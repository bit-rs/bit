/// Imports
use crate::cx::root::RootCx;
use bit_common::package::DraftPackage;

/// Package ctx
pub struct PackageCx<'cx> {
    /// Draft package
    pub draft: DraftPackage,
    /// Root cx
    pub root: &'cx mut RootCx,
}
