/// Imports
use crate::{
    cx::module::ModuleCx,
    typ::{
        def::TypeDef,
        typ::{EnumVariant, Field},
    },
};
use bit_ast::ast::{self, EnumConstructor, TypeDeclaration};
use bit_common::span::Span;
use ecow::EcoString;

/// Late declaration analysis pass for the module.
///
/// This pass completes the semantic analysis of declarations (structs, enums,
/// functions, constants) after their names and initial shells
/// have been registered during the early phase.
///
/// In this stage:
/// - Generic parameters are reinstated into the inference context.
/// - All type annotations are resolved into `Typ`.
/// - Function bodies are type-checked.
/// - Struct and enum fields are fully typed.
/// - Constants are inferred and unified against their annotations.
/// - All definitions are finalized and registered into the resolver.
///
impl<'pkg, 'cx> ModuleCx<'pkg, 'cx> {
    /// Performs late analysis of a struct declaration.
    ///
    /// ## Responsibilities:
    /// - Infer the types of all fields using `infer_type_annotation`.
    /// - Rebuild the `Struct` def with resolved field types.
    /// - Overwrite the existing struct definition with the completed one.
    ///
    /// This operation mutates the struct in place, finalizing its type
    /// structure for the rest of type checking.
    ///
    fn late_analyze_struct(
        &mut self,
        location: Span,
        name: EcoString,
        generics: Vec<EcoString>,
        fields: Vec<ast::Field>,
    ) {
        // Requesting struct
        let id = match self.resolver.resolve_type(&location, &name) {
            TypeDef::Struct(ty) => ty,
            _ => unreachable!(),
        };

        // Inferring fields
        let fields = {
            // Re pushing generics
            self.icx.generics.push_scope(generics);

            let fields = fields
                .into_iter()
                .map(|f| Field {
                    name: f.name,
                    location: f.location,
                    typ: self.infer_type_annotation(f.typ),
                })
                .collect::<Vec<Field>>();

            // Popping generics
            self.icx.generics.pop_scope();

            fields
        };

        // Setting fields
        let struct_mut = self.icx.tcx.struct_mut(id);
        struct_mut.fields = fields;
    }

    /// Performs late analysis of an enum declaration.
    ///
    /// ## Responsibilities:
    /// - Infer the types of all variant fields.
    /// - Rebuild the `Enum` def with resolved variant field types.
    /// - Overwrite the existing enum definition with the completed one.
    ///
    /// Enum variant fields are treated similarly to struct fields: each
    /// parameter is analyzed using `infer_type_annotation`.
    ///
    fn late_analyze_enum(
        &mut self,
        location: Span,
        name: EcoString,
        generics: Vec<EcoString>,
        variants: Vec<EnumConstructor>,
    ) {
        // Requesting enum
        let id = match self.resolver.resolve_type(&location, &name) {
            TypeDef::Enum(en) => en,
            _ => unreachable!(),
        };

        // Inferring variants
        let variants = {
            // Repushing generics
            self.icx.generics.push_scope(generics);

            let variants = variants
                .into_iter()
                .map(|v| EnumVariant {
                    location: v.location,
                    name: v.name,
                    fields: v
                        .params
                        .into_iter()
                        .map(|p: ast::Parameter| Field {
                            location: p.location,
                            name: p.name,
                            typ: self.infer_type_annotation(p.typ),
                        })
                        .collect(),
                })
                .collect::<Vec<EnumVariant>>();

            // Popping generics
            self.icx.generics.pop_scope();

            variants
        };

        // Setting variants
        let enum_mut = self.icx.tcx.enum_mut(id);
        enum_mut.variants = variants;
    }

    /// Dispatches a type declaration to the corresponding late analysis routine.
    ///
    /// Each type declaration variant is fully processed here:
    /// - Struct → `late_analyze_struct`
    /// - Enum → `late_analyze_enum`
    ///
    /// After this call, each type declaration is fully type-analyzed and integrated
    /// into the module’s type environment.
    ///
    pub fn late_analyze_type_decl(&mut self, decl: TypeDeclaration) {
        match decl {
            TypeDeclaration::Struct {
                location,
                name,
                fields,
                generics,
                ..
            } => self.late_analyze_struct(location, name, generics, fields),
            TypeDeclaration::Enum {
                location,
                name,
                variants,
                generics,
                ..
            } => self.late_analyze_enum(location, name, generics, variants),
        }
    }
}
