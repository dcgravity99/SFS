/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashSet;
use std::sync::RwLock;

pub struct AssetTypeRegistry {
    types: RwLock<HashSet<String>>,
}

impl AssetTypeRegistry {
    pub fn new() -> Self {
        let mut set = HashSet::new();
        let builtins = ["Video", "Audio", "Image", "Character", "Actor", "Voice", "Location", "Prop", "Style", "Prompt", "Storyboard", "Script"];
        for b in builtins {
            set.insert(b.to_string());
        }

        Self {
            types: RwLock::new(set),
        }
    }

    pub fn register_plugin_type(&self, plugin_type: &str) -> bool {
        if let Ok(mut set) = self.types.write() {
            set.insert(plugin_type.to_string())
        } else {
            false
        }
    }

    pub fn is_valid_type(&self, asset_type: &str) -> bool {
        if let Ok(set) = self.types.read() {
            set.contains(asset_type)
        } else {
            false
        }
    }
}
