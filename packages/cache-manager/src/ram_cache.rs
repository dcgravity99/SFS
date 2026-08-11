/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;
use std::sync::RwLock;

pub struct RamCacheTier {
    entries: RwLock<HashMap<String, Vec<u8>>>,
}

impl RamCacheTier {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        if let Ok(map) = self.entries.read() {
            map.get(key).cloned()
        } else {
            None
        }
    }

    pub fn put(&self, key: &str, data: Vec<u8>) {
        if let Ok(mut map) = self.entries.write() {
            map.insert(key.to_string(), data);
        }
    }
}
