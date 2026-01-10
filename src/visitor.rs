use std::collections::HashMap;
use swc_core::common::{Mark, SyntaxContext, DUMMY_SP};
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitWith},
};

use crate::collector::IdentifierCollector;
use crate::config::{ImportConfig, ImportItem, ImportSource, PluginConfig};
use crate::presets::get_preset_imports;

/// Main transform visitor
pub struct AutoImportVisitor {
    /// Import map: source -> [(name, alias)]
    import_map: HashMap<String, Vec<(String, Option<String>)>>,
    /// Unresolved mark for proper syntax context
    unresolved_mark: Mark,
    /// Enable debug logging
    debug: bool,
}

impl AutoImportVisitor {
    pub fn new(config: PluginConfig, unresolved_mark: Mark) -> Self {
        let debug = config.debug;
        let mut import_map = HashMap::new();

        // Process imports array
        for import_config in &config.imports {
            match import_config {
                // Simple string form: "react" - treat as preset
                ImportConfig::Simple(preset) => {
                    let preset_imports = get_preset_imports(preset);
                    for (source, imports) in preset_imports {
                        import_map
                            .entry(source)
                            .or_insert_with(Vec::new)
                            .extend(imports);
                    }
                }
                // Explicit form: { from: "...", imports: [...] }
                ImportConfig::Explicit { from, imports } => {
                    let import_list: Vec<(String, Option<String>)> = match imports {
                        ImportSource::Simple(names) => {
                            names.iter().map(|name| (name.clone(), None)).collect()
                        }
                        ImportSource::WithAlias(items) => items
                            .iter()
                            .map(|item| match item {
                                ImportItem::Simple(name) => (name.clone(), None),
                                ImportItem::Aliased([name, alias]) => {
                                    (name.clone(), Some(alias.clone()))
                                }
                            })
                            .collect(),
                    };

                    import_map
                        .entry(from.clone())
                        .or_insert_with(Vec::new)
                        .extend(import_list);
                }
                // Object mapping form: { "package": ["export1", "export2"] }
                ImportConfig::Mapping(map) => {
                    for (source, import_source) in map {
                        let import_list: Vec<(String, Option<String>)> = match import_source {
                            ImportSource::Simple(names) => {
                                names.iter().map(|name| (name.clone(), None)).collect()
                            }
                            ImportSource::WithAlias(items) => items
                                .iter()
                                .map(|item| match item {
                                    ImportItem::Simple(name) => (name.clone(), None),
                                    ImportItem::Aliased([name, alias]) => {
                                        (name.clone(), Some(alias.clone()))
                                    }
                                })
                                .collect(),
                        };

                        import_map
                            .entry(source.clone())
                            .or_insert_with(Vec::new)
                            .extend(import_list);
                    }
                }
            }
        }

        Self {
            import_map,
            unresolved_mark,
            debug,
        }
    }

    /// Add auto imports to the module
    fn add_auto_imports(&self, module: &mut Module) {
        // Collect identifier information
        let mut collector = IdentifierCollector::new();
        module.visit_with(&mut collector);

        // Debug: print collected identifiers (only if debug is enabled)
        if self.debug {
            eprintln!(
                "[DEBUG] Used identifiers count: {}",
                collector.used_identifiers.len()
            );
            eprintln!(
                "[DEBUG] Used identifiers (sample): {:?}",
                collector
                    .used_identifiers
                    .iter()
                    .take(10)
                    .collect::<Vec<_>>()
            );

            eprintln!(
                "[DEBUG] Imported identifiers count: {}",
                collector.imported_identifiers.len()
            );
            eprintln!(
                "[DEBUG] Imported identifiers (sample): {:?}",
                collector
                    .imported_identifiers
                    .iter()
                    .take(10)
                    .collect::<Vec<_>>()
            );

            eprintln!(
                "[DEBUG] Declared identifiers count: {}",
                collector.declared_identifiers.len()
            );
            eprintln!(
                "[DEBUG] Declared identifiers (sample): {:?}",
                collector
                    .declared_identifiers
                    .iter()
                    .take(10)
                    .collect::<Vec<_>>()
            );
        }

        // Find identifiers that need to be auto-imported
        let mut imports_to_add: HashMap<String, Vec<(String, Option<String>)>> = HashMap::new();

        for (source, available_imports) in &self.import_map {
            for (name, alias) in available_imports {
                let local_name = alias.as_ref().unwrap_or(name);

                // If identifier is used but not imported or declared, add import
                if collector.used_identifiers.contains(local_name)
                    && !collector.imported_identifiers.contains(local_name)
                    && !collector.declared_identifiers.contains(local_name)
                {
                    imports_to_add
                        .entry(source.clone())
                        .or_insert_with(Vec::new)
                        .push((name.clone(), alias.clone()));
                }
            }
        }

        // Generate import statements with sorted order
        let mut new_imports = Vec::new();

        // Sort sources alphabetically for consistent order
        let mut sorted_sources: Vec<_> = imports_to_add.into_iter().collect();
        sorted_sources.sort_by(|a, b| a.0.cmp(&b.0));

        // Create the unresolved context for imported identifiers
        let unresolved_ctxt = SyntaxContext::empty().apply_mark(self.unresolved_mark);

        for (source, mut imports) in sorted_sources {
            // Sort imports within each source alphabetically by name
            imports.sort_by(|a, b| a.0.cmp(&b.0));

            let specifiers = imports
                .into_iter()
                .map(|(name, alias)| {
                    match alias {
                        Some(alias_name) => {
                            // Import with alias: import { name as alias_name } from "source"
                            ImportSpecifier::Named(ImportNamedSpecifier {
                                span: DUMMY_SP,
                                local: Ident::new(
                                    alias_name.clone().into(),
                                    DUMMY_SP,
                                    unresolved_ctxt,
                                ),
                                imported: Some(ModuleExportName::Ident(Ident::new(
                                    name.into(),
                                    DUMMY_SP,
                                    unresolved_ctxt,
                                ))),
                                is_type_only: false,
                            })
                        }
                        None => {
                            // Simple import: import { name } from "source"
                            // Both local and imported should be None (or same value)
                            ImportSpecifier::Named(ImportNamedSpecifier {
                                span: DUMMY_SP,
                                local: Ident::new(name.clone().into(), DUMMY_SP, unresolved_ctxt),
                                imported: None,
                                is_type_only: false,
                            })
                        }
                    }
                })
                .collect();

            new_imports.push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers,
                src: Box::new(Str {
                    span: DUMMY_SP,
                    value: source.into(),
                    raw: None,
                }),
                type_only: false,
                with: None,
                phase: Default::default(),
            })));
        }

        // Add new imports to the top of the module, after any directives
        if !new_imports.is_empty() {
            // Find the position after any directives (like "use client", "use server")
            let insert_position = module
                .body
                .iter()
                .position(|item| {
                    // Stop at the first non-directive item
                    match item {
                        ModuleItem::Stmt(Stmt::Expr(ExprStmt { expr, .. })) => {
                            !matches!(**expr, Expr::Lit(Lit::Str(_)))
                        }
                        _ => true,
                    }
                })
                .unwrap_or(module.body.len());

            // Insert imports at the calculated position
            let mut items = module.body.drain(..insert_position).collect::<Vec<_>>();
            items.extend(new_imports);
            items.extend(module.body.drain(..));
            module.body = items;
        }
    }
}

impl VisitMut for AutoImportVisitor {
    fn visit_mut_module(&mut self, module: &mut Module) {
        self.add_auto_imports(module);
    }
}
