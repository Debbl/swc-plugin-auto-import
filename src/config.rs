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

/// Import configuration item - supports multiple formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportConfig {
    /// Simple string form: "react"
    Simple(String),
    /// Explicit form with individual import items, each with their own from field
    /// Example: [{ name: "ref", from: "vue" }, { name: "useState", as: "useSignal", from: "react" }]
    Explicit(Vec<ExplicitImport>),
    /// Object mapping form - simplified syntax for package imports
    ///
    /// Supports:
    /// - Named imports: { "@vueuse/core": ["useMouse"] } -> import { useMouse } from '@vueuse/core'
    /// - Aliased imports: { "@vueuse/core": [["useFetch", "useMyFetch"]] } -> import { useFetch as useMyFetch } from '@vueuse/core'
    /// - Default imports: { "axios": [["default", "axios"]] } -> import axios from 'axios'
    /// - Namespace imports: { "lodash": [["*", "_"]] } -> import * as _ from 'lodash'
    Mapping(HashMap<String, ImportSource>),
}

/// Plugin configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Import configurations: can be strings, objects with 'from' field, or package mappings
    #[serde(default)]
    pub imports: Vec<ImportConfig>,

    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,
}
