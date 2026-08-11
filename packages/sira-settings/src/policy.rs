/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashSet;

#[derive(Default)]
pub struct SettingPolicyLocks {
    locked_keys: HashSet<String>,
}

impl SettingPolicyLocks {
    pub fn is_locked(&self, key: &str) -> bool {
        self.locked_keys.contains(key)
    }

    pub fn lock(&mut self, key: &str) {
        self.locked_keys.insert(key.to_string());
    }
}
