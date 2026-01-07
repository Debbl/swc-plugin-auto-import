use std::collections::HashSet;
use swc_core::ecma::{
    ast::*,
    visit::{Visit, VisitWith},
};

/// Identifier collector - collects all identifiers used in the code
pub struct IdentifierCollector {
    /// Used identifiers
    pub used_identifiers: HashSet<String>,
    /// Declared identifiers (functions, variables, classes, etc.)
    pub declared_identifiers: HashSet<String>,
    /// Imported identifiers
    pub imported_identifiers: HashSet<String>,
}

impl IdentifierCollector {
    pub fn new() -> Self {
        Self {
            used_identifiers: HashSet::new(),
            declared_identifiers: HashSet::new(),
            imported_identifiers: HashSet::new(),
        }
    }
}

impl Visit for IdentifierCollector {
    // Collect imported identifiers
    fn visit_import_decl(&mut self, import: &ImportDecl) {
        for specifier in &import.specifiers {
            match specifier {
                ImportSpecifier::Named(named) => {
                    let local = &named.local.sym;
                    self.imported_identifiers.insert(local.to_string());
                }
                ImportSpecifier::Default(default) => {
                    let local = &default.local.sym;
                    self.imported_identifiers.insert(local.to_string());
                }
                ImportSpecifier::Namespace(ns) => {
                    let local = &ns.local.sym;
                    self.imported_identifiers.insert(local.to_string());
                }
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
    }

    // Collect declared identifiers
    fn visit_var_declarator(&mut self, declarator: &VarDeclarator) {
        if let Pat::Ident(ident) = &declarator.name {
            self.declared_identifiers.insert(ident.sym.to_string());
        }
        declarator.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, func: &FnDecl) {
        self.declared_identifiers.insert(func.ident.sym.to_string());
        func.visit_children_with(self);
    }

    fn visit_class_decl(&mut self, class: &ClassDecl) {
        self.declared_identifiers
            .insert(class.ident.sym.to_string());
        class.visit_children_with(self);
    }

    // Collect used identifiers
    fn visit_ident(&mut self, ident: &Ident) {
        self.used_identifiers.insert(ident.sym.to_string());
    }
}
