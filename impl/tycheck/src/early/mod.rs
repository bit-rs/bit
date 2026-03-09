/// Imports
use crate::check::ModuleTyck;

/// Implementation
impl<'tcx, 'icx> ModuleTyck<'tcx, 'icx> {
    /// Performs imports 
    fn early_define_item(&mut self, item: &ast::item::Item) {
        match item.kind {
            ast::item::ItemKind::Struct(_) => todo!(),
            ast::item::ItemKind::Enum(_) => todo!(),
            _ => {}
        }
    }

    /// Early analysis phase: performs imports
    pub fn early_phase(&mut self, m: &ast::item::Module) {
        for item in &m.items {
            self.early_define_item(item);
        }
    }
}
