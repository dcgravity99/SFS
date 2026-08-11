/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use async_trait::async_trait;
use sira_types::SiraResult;
use sira_core::capabilities::AICapability;
use crate::manifest::{ProviderManifest, AIModelInfo};
use crate::contracts::{AIRequest, AIResponse, AIUsage};
use crate::provider_trait::AiProvider;

pub struct MockProvider;

#[async_trait]
impl AiProvider for MockProvider {
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest {
            provider_id: "provider-mock".to_string(),
            vendor_name: "Siragugal Internal Verification Mock".to_string(),
            supported_capabilities: vec![AICapability::TextGeneration, AICapability::VideoGeneration],
            models: vec![AIModelInfo {
                model_id: "mock-model-v1".to_string(),
                display_name: "Mock Verification Model v1".to_string(),
                context_window_tokens: 4096,
                vram_required_mb: 512,
                checksum_sha256: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            }],
            supports_streaming: true,
            auth_type: "None".to_string(),
            license: "Apache-2.0".to_string(),
            is_offline_capable: true,
        }
    }

    fn estimate_cost(&self, _request: &AIRequest) -> SiraResult<f64> {
        SiraResult::Success(0.0)
    }

    async fn execute(&self, request: AIRequest) -> SiraResult<AIResponse> {
        SiraResult::Success(AIResponse {
            request_id: request.request_id,
            output_text: Some("Verification Mock Output".to_string()),
            output_media_uri: None,
            usage: AIUsage {
                prompt_tokens: 10,
                completion_tokens: 10,
                total_tokens: 20,
                cost_usd: 0.0,
            },
        })
    }
}
