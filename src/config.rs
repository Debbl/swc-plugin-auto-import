use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Import source configuration - defines which APIs to import from which packages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportSource {
    /// Simple array form: ["useMouse", "useFetch"]
    Simple(Vec<String>),
    /// With alias form: [["useFetch", "useMyFetch"]]
    WithAlias(Vec<ImportItem>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportItem {
    Simple(String),
    Aliased([String; 2]),
}

/// Explicit import item with name, optional alias, and from field
///
/// Examples:
/// - Named import: { name: "ref", from: "vue" } -> import { ref } from 'vue'
/// - Named import with alias: { name: "useState", as: "useSignal", from: "react" } -> import { useState as useSignal } from 'react'
/// - Default import: { name: "default", as: "_", from: "lodash" } -> import _ from 'lodash'
/// - Namespace import: { name: "*", as: "lodash", from: "lodash" } -> import * as lodash from 'lodash'
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplicitImport {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "as")]
    pub alias: Option<String>,
    pub from: String,
}

/// PresetImport - supports multiple formats for inline preset imports
///
/// Corresponds to: Omit<Import, 'from'> | ImportName | [name: ImportName, as?: ImportName, from?: ModuleId]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PresetImport {
    /// Simple string: "useState"
    Simple(String),
    /// Tuple form: ["useState", "useSignal"] or ["useState", "useSignal", "react"]
    Tuple(Vec<String>),
    /// Object form: { name: "useState", as?: "useSignal" }
    Object {
        name: String,
        #[serde(rename = "as")]
        #[serde(skip_serializing_if = "Option::is_none")]
        alias: Option<String>,
    },
    /// Nested inline preset
    Nested(Box<InlinePreset>),
}

/// InlinePreset - defines imports from a single module
///
/// Corresponds to: { from: ModuleId, type?: boolean, imports: (PresetImport | InlinePreset)[] }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlinePreset {
    /// Module specifier to import from
    pub from: String,
    /// If this import is a pure type import
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_only: Option<bool>,
    /// List of imports from this module
    pub imports: Vec<PresetImport>,
}

/// Import configuration item - supports multiple formats
///
/// Corresponds to: ImportsMap | PresetName | InlinePreset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportConfig {
    /// InlinePreset: { from: "react", imports: ["useState", "useEffect"] }
    /// Must be checked first due to the 'from' field being the distinguishing factor
    InlinePreset(InlinePreset),

    /// Explicit form with individual import items (legacy support)
    /// Example: [{ name: "ref", from: "vue" }, { name: "useState", as: "useSignal", from: "react" }]
    Explicit(Vec<ExplicitImport>),

    /// ImportsMap: Object mapping form - simplified syntax for package imports
    ///
    /// Supports:
    /// - Named imports: { "@vueuse/core": ["useMouse"] } -> import { useMouse } from '@vueuse/core'
    /// - Aliased imports: { "@vueuse/core": [["useFetch", "useMyFetch"]] } -> import { useFetch as useMyFetch } from '@vueuse/core'
    /// - Default imports: { "axios": [["default", "axios"]] } -> import axios from 'axios'
    /// - Namespace imports: { "lodash": [["*", "_"]] } -> import * as _ from 'lodash'
    ImportsMap(HashMap<String, ImportSource>),

    /// PresetName: Simple string form like "react", "vue", "react-dom"
    PresetName(String),
}

/// Arrayable type - supports both single value and array
///
/// Corresponds to: type Arrayable<T> = T | Array<T>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Arrayable<T> {
    Single(T),
    Array(Vec<T>),
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Import configurations: can be a single item or array of ImportsMap | PresetName | InlinePreset
    ///
    /// Corresponds to: imports?: Arrayable<ImportsMap | PresetName | InlinePreset>
    #[serde(default)]
    pub imports: Option<Arrayable<ImportConfig>>,

    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            imports: None,
            debug: false,
        }
    }
}
