/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;
use std::sync::RwLock;

pub struct FeatureFlagManager {
    flags: RwLock<HashMap<String, bool>>,
}

impl FeatureFlagManager {
    pub fn new() -> Self {
        Self {
            flags: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_flag(&self, flag: &str, enabled: bool) {
        if let Ok(mut map) = self.flags.write() {
            map.insert(flag.to_string(), enabled);
        }
    }

    pub fn is_enabled(&self, flag: &str, default_value: bool) -> bool {
        if let Ok(map) = self.flags.read() {
            *map.get(flag).unwrap_or(&default_value)
        } else {
            default_value
        }
    }
}
