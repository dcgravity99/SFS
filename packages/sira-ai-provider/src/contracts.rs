/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_core::capabilities::AICapability;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub cost_usd: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIRequest {
    pub request_id: String,
    pub capability: AICapability,
    pub model_id: String,
    pub prompt: String,
    pub parameters_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub request_id: String,
    pub output_text: Option<String>,
    pub output_media_uri: Option<String>,
    pub usage: AIUsage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIStreamChunk {
    pub request_id: String,
    pub chunk_index: usize,
    pub text_delta: Option<String>,
    pub progress_percent: Option<f32>,
    pub is_final: bool,
}
