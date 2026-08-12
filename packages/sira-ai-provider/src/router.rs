/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::manifest::ProviderManifest;
use sira_core::capabilities::AICapability;
use sira_types::SiraResult;

pub struct ProviderRouter;

impl ProviderRouter {
    pub fn select_offline_first_fallback_chain(
        capability: AICapability,
    ) -> SiraResult<Vec<String>> {
        let _ = capability;
        // Offline-First preference order: Local Model -> Enterprise Server -> Cloud -> Fallback
        SiraResult::Success(vec![
            "provider-local-llm".to_string(),
            "provider-cloud-openai".to_string(),
            "provider-mock".to_string(),
        ])
    }
}
