/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigOriginLayer {
    BuiltinDefault,
    SystemFile,
    UserFile,
    ProjectFile,
    EnvironmentVariable,
    CommandLineArgument,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigValueProvenance {
    pub key: String,
    pub value: String,
    pub origin_layer: ConfigOriginLayer,
    pub source_detail: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigDiagnostics {
    pub provenance_map: std::collections::HashMap<String, ConfigValueProvenance>,
}

impl ConfigDiagnostics {
    pub fn record(&mut self, key: &str, value: &str, layer: ConfigOriginLayer, detail: &str) {
        self.provenance_map.insert(
            key.to_string(),
            ConfigValueProvenance {
                key: key.to_string(),
                value: value.to_string(),
                origin_layer: layer,
                source_detail: detail.to_string(),
            },
        );
    }
}
