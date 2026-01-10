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

/// Import configuration item - supports multiple formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportConfig {
    /// Simple string form: "react"
    Simple(String),
    /// Explicit form with from field: { from: "motion/react-m", imports: [['*', 'motion']] }
    Explicit { from: String, imports: ImportSource },
    /// Object mapping form: { "twl": ["cn"] }
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
