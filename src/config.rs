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

/// Plugin configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Presets: ["vue", "react", "vue-router"]
    #[serde(default)]
    pub presets: Vec<String>,

    /// Custom import mappings: { "@vueuse/core": ["useMouse", "useFetch"] }
    #[serde(default)]
    pub imports: HashMap<String, ImportSource>,

    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,
}
