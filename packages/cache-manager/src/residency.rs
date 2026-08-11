/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashSet;

pub struct ModelResidencyManager {
    pinned_models: HashSet<String>,
}

impl ModelResidencyManager {
    pub fn new() -> Self {
        Self {
            pinned_models: HashSet::new(),
        }
    }

    pub fn pin_model(&mut self, model_id: &str) {
        self.pinned_models.insert(model_id.to_string());
    }

    pub fn is_pinned(&self, model_id: &str) -> bool {
        self.pinned_models.contains(model_id)
    }
}
