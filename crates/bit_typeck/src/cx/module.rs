/// Imports
use crate::{
    cx::package::PackageCx,
    resolve::resolve::ModuleResolver,
    typ::{
        cx::{InferCx, TyCx},
        typ::Module,
    },
};
use ecow::EcoString;
use bit_ast::ast::{self};

/// Module ctx
pub struct ModuleCx<'pkg, 'cx> {
    /// Current analyzing module info
    pub(crate) module: &'pkg ast::Module,
    pub(crate) module_name: &'pkg EcoString,
    /// Resolver
    pub(crate) resolver: ModuleResolver,
    /// Inference context
    pub(crate) icx: InferCx<'cx>,
    /// Root package context
    pub(crate) package: &'cx PackageCx<'cx>,
}

/// Implementation
impl<'pkg, 'cx> ModuleCx<'pkg, 'cx> {
    /// Creates new module analyzer
    pub fn new(
        module: &'pkg ast::Module,
        module_name: &'pkg EcoString,
        tcx: &'cx mut TyCx,
        package: &'cx PackageCx<'pkg>,
    ) -> Self {
        Self {
            module,
            module_name,
            resolver: ModuleResolver::default(),
            icx: InferCx::new(tcx),
            package,
        }
    }

    /// Performs analyze of module
    pub fn analyze(&mut self) -> Module {
        self.pipeline()
    }
}
