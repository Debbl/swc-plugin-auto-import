use std::sync::Arc;
use swc_core::common::SourceMap;
use swc_core::ecma::{
    ast::Program, codegen::text_writer::JsWriter, codegen::Emitter, visit::VisitMutWith,
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

mod collector;
mod config;
mod presets;
mod visitor;

pub use config::{
    Arrayable, ExplicitImport, ImportConfig, ImportItem, ImportSource, InlinePreset, PluginConfig,
    PresetImport,
};
pub use visitor::AutoImportVisitor;

/// Convert Program AST to source code string for debugging
fn program_to_string(program: &Program) -> String {
    let cm = Arc::new(SourceMap::default());
    let mut buf = vec![];
    {
        let mut emitter = Emitter {
            cfg: swc_core::ecma::codegen::Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        let _ = emitter.emit_program(program);
    }
    String::from_utf8_lossy(&buf).to_string()
}

/// SWC plugin entry point - auto import transformation
///
/// Usage:
/// .swcrc configuration:
/// ```json
/// {
///   "jsc": {
///     "experimental": {
///       "plugins": [
///         ["swc-plugin-auto-import", {
///           "imports": [
///             "react",
///             "react-dom",
///             {
///               "@vueuse/core": ["useMouse", ["useFetch", "useMyFetch"]],
///               "axios": [["default", "axios"]],
///               "lodash": [["*", "_"]]
///             },
///             [
///               { "name": "ref", "from": "vue" },
///               { "name": "useState", "as": "useSignal", "from": "react" },
///               { "name": "default", "as": "axios", "from": "axios" },
///               { "name": "*", "as": "motion", "from": "motion/react-m" }
///             ]
///           ]
///         }]
///       ]
///     }
///   }
/// }
/// ```
#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    // Parse configuration
    let config_str = metadata
        .get_transform_plugin_config()
        .unwrap_or_else(|| "{}".to_string());
    let config = serde_json::from_str::<PluginConfig>(&config_str).unwrap_or_default();

    let debug = config.debug;

    // Print input source code (only if debug is enabled)
    if debug {
        eprintln!("\n========== INPUT SOURCE CODE ==========");
        eprintln!("{}", program_to_string(&program));
        eprintln!("=======================================\n");
    }

    // Get unresolved_mark for proper syntax context
    let unresolved_mark = metadata.unresolved_mark;

    // Apply transformation only to Module programs
    match &mut program {
        Program::Module(module) => {
            let mut visitor = AutoImportVisitor::new(config, unresolved_mark);
            module.visit_mut_with(&mut visitor);
        }
        _ => {
            // Scripts don't support imports, skip transformation
        }
    }

    // Print output source code (only if debug is enabled)
    if debug {
        eprintln!("\n========== OUTPUT SOURCE CODE ==========");
        eprintln!("{}", program_to_string(&program));
        eprintln!("========================================\n");
    }

    program
}
