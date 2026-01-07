use std::{path::PathBuf, sync::Arc};
use swc_core::{
    common::{Mark, SourceMap, GLOBALS},
    ecma::{
        ast::*,
        codegen::{text_writer::JsWriter, Config as CodegenConfig, Emitter},
        parser::{parse_file_as_module, Syntax, TsSyntax},
        visit::VisitMutWith,
    },
};
use swc_plugin_auto_import::{AutoImportVisitor, PluginConfig};

#[testing::fixture("tests/fixture/**/input.ts")]
#[testing::fixture("tests/fixture/**/input.tsx")]
fn fixture(input: PathBuf) {
    let output_path = input.with_file_name("output.js");
    let config_path = input.with_file_name("config.json");

    // Read config from config.json file
    let config = if config_path.exists() {
        let config_str = std::fs::read_to_string(&config_path).expect("Failed to read config.json");
        serde_json::from_str::<PluginConfig>(&config_str).expect("Failed to parse config.json")
    } else {
        PluginConfig::default()
    };

    // Read expected output
    let expected = std::fs::read_to_string(&output_path).expect("Failed to read output.js");

    // Parse input
    let cm = Arc::new(SourceMap::default());
    let fm = cm.load_file(&input).expect("Failed to load input file");

    let is_tsx = input.to_string_lossy().ends_with(".tsx");
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: is_tsx,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
    });

    GLOBALS.set(&Default::default(), || {
        let mut module = parse_file_as_module(&fm, syntax, EsVersion::Es2020, None, &mut vec![])
            .expect("Failed to parse input");

        // Apply transform
        let unresolved_mark = Mark::new();
        let mut visitor = AutoImportVisitor::new(config, unresolved_mark);
        module.visit_mut_with(&mut visitor);

        // Generate output
        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: CodegenConfig::default(),
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };
            emitter.emit_module(&module).expect("Failed to emit");
        }

        let actual = String::from_utf8(buf).expect("Invalid UTF-8");

        // Compare output
        if actual.trim() != expected.trim() {
            panic!(
                "\n\n========== EXPECTED ==========\n{}\n========== ACTUAL ==========\n{}\n",
                expected.trim(),
                actual.trim()
            );
        }
    });
}
