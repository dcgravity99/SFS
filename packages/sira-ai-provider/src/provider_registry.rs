/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::provider_trait::AiProvider;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct ProviderRegistry {
    providers: RwLock<HashMap<String, Box<dyn AiProvider>>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, provider: Box<dyn AiProvider>) {
        let id = provider.manifest().provider_id;
        if let Ok(mut map) = self.providers.write() {
            map.insert(id, provider);
        }
    }
}
