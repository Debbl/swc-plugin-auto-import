use std::collections::HashMap;
use swc_core::common::{Mark, SyntaxContext, DUMMY_SP};
use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitWith},
};

use crate::collector::IdentifierCollector;
use crate::config::{
    Arrayable, ImportConfig, ImportItem, ImportSource, InlinePreset, PluginConfig, PresetImport,
};
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

        // Process imports (Option<Arrayable<ImportConfig>>)
        if let Some(imports) = config.imports {
            // Convert Arrayable to Vec for iteration
            let imports_vec = match imports {
                Arrayable::Single(item) => vec![item],
                Arrayable::Array(items) => items,
            };

            for import_config in imports_vec {
                Self::process_import_config(&mut import_map, import_config);
            }
        }

        Self {
            import_map,
            unresolved_mark,
            debug,
        }
    }

    /// Process a single ImportConfig and add to import_map
    fn process_import_config(
        import_map: &mut HashMap<String, Vec<(String, Option<String>)>>,
        import_config: ImportConfig,
    ) {
        match import_config {
            // PresetName: Simple string form like "react", "vue", "react-dom"
            ImportConfig::PresetName(preset) => {
                let preset_imports = get_preset_imports(&preset);
                for (source, imports) in preset_imports {
                    import_map
                        .entry(source)
                        .or_insert_with(Vec::new)
                        .extend(imports);
                }
            }
            // InlinePreset: { from: "react", imports: ["useState", "useEffect"] }
            ImportConfig::InlinePreset(inline_preset) => {
                Self::process_inline_preset(import_map, inline_preset);
            }
            // Explicit form (legacy): [{ name: "ref", from: "vue" }, ...]
            ImportConfig::Explicit(items) => {
                for item in items {
                    import_map
                        .entry(item.from.clone())
                        .or_insert_with(Vec::new)
                        .push((item.name.clone(), item.alias.clone()));
                }
            }
            // ImportsMap: { "package": ["export1", "export2"] }
            ImportConfig::ImportsMap(map) => {
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

    /// Process an InlinePreset and add to import_map
    fn process_inline_preset(
        import_map: &mut HashMap<String, Vec<(String, Option<String>)>>,
        inline_preset: InlinePreset,
    ) {
        let source = inline_preset.from;

        for preset_import in inline_preset.imports {
            Self::process_preset_import(import_map, preset_import, &source);
        }
    }

    /// Process a PresetImport and add to import_map
    fn process_preset_import(
        import_map: &mut HashMap<String, Vec<(String, Option<String>)>>,
        preset_import: PresetImport,
        default_source: &str,
    ) {
        match preset_import {
            // Simple string: "useState"
            PresetImport::Simple(name) => {
                import_map
                    .entry(default_source.to_string())
                    .or_insert_with(Vec::new)
                    .push((name, None));
            }
            // Tuple: ["useState", "useSignal"] or ["useState", "useSignal", "react"]
            PresetImport::Tuple(parts) => {
                let (name, alias, source) = match parts.len() {
                    1 => (parts[0].clone(), None, default_source.to_string()),
                    2 => (
                        parts[0].clone(),
                        Some(parts[1].clone()),
                        default_source.to_string(),
                    ),
                    3 => (parts[0].clone(), Some(parts[1].clone()), parts[2].clone()),
                    _ => return, // Invalid tuple length
                };

                import_map
                    .entry(source)
                    .or_insert_with(Vec::new)
                    .push((name, alias));
            }
            // Object: { name: "useState", as?: "useSignal" }
            PresetImport::Object { name, alias } => {
                import_map
                    .entry(default_source.to_string())
                    .or_insert_with(Vec::new)
                    .push((name, alias));
            }
            // Nested InlinePreset
            PresetImport::Nested(nested) => {
                Self::process_inline_preset(import_map, *nested);
            }
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
                    // Handle default imports: { name: "default", as: "_", from: "lodash" }
                    if name == "default" {
                        // Default import: import alias_name from "source"
                        let local_name = alias.unwrap_or_else(|| "default".to_string());
                        return ImportSpecifier::Default(ImportDefaultSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new(local_name.into(), DUMMY_SP, unresolved_ctxt),
                        });
                    }

                    // Handle namespace imports: { name: "*", as: "_", from: "lodash" }
                    if name == "*" {
                        // Namespace import: import * as alias_name from "source"
                        let local_name = alias.unwrap_or_else(|| "default".to_string());
                        return ImportSpecifier::Namespace(ImportStarAsSpecifier {
                            span: DUMMY_SP,
                            local: Ident::new(local_name.into(), DUMMY_SP, unresolved_ctxt),
                        });
                    }

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
