/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashSet;
use std::sync::RwLock;

pub struct PluginCapabilityRegistry {
    registered_capabilities: RwLock<HashSet<String>>,
}

impl PluginCapabilityRegistry {
    pub fn new() -> Self {
        Self {
            registered_capabilities: RwLock::new(HashSet::new()),
        }
    }

    pub fn register(&self, capability: &str) {
        if let Ok(mut set) = self.registered_capabilities.write() {
            set.insert(capability.to_string());
        }
    }
}
