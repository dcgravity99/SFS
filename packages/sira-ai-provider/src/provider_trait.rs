/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use async_trait::async_trait;
use sira_types::SiraResult;
use crate::manifest::ProviderManifest;
use crate::contracts::{AIRequest, AIResponse};

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn manifest(&self) -> ProviderManifest;
    fn estimate_cost(&self, request: &AIRequest) -> SiraResult<f64>;
    async fn execute(&self, request: AIRequest) -> SiraResult<AIResponse>;
}
