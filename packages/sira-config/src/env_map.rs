/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;

pub fn map_sira_env_vars() -> HashMap<String, String> {
    let mut overrides = HashMap::new();
    for (key, val) in std::env::vars() {
        if key.starts_with("SIRA_") {
            let normalized_key = key
                .trim_start_matches("SIRA_")
                .to_lowercase()
                .replace('_', ".");
            overrides.insert(normalized_key, val);
        }
    }
    overrides
}
