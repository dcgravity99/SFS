/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::path::{Path, PathBuf};
use async_trait::async_trait;
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use sira_core::capabilities::AICapability;

use crate::manifest::{ProviderManifest, AIModelInfo};
use crate::contracts::{AIRequest, AIResponse, AIUsage};
use crate::provider_trait::AiProvider;
use crate::model_registry::ModelRegistry;

pub struct CandleLlmProvider {
    model_id: String,
    display_name: String,
    model_path: PathBuf,
    expected_sha256: Option<String>,
}

impl CandleLlmProvider {
    pub fn new(model_path: PathBuf, expected_sha256: Option<String>) -> Self {
        let filename = model_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("candle-gguf-model")
            .to_string();

        Self {
            model_id: format!("candle-llm-{}", filename),
            display_name: format!("Candle Local GGUF ({})", filename),
            model_path,
            expected_sha256,
        }
    }

    pub fn model_path(&self) -> &Path {
        &self.model_path
    }

    pub fn verify_weights(&self) -> SiraResult<bool> {
        if !self.model_path.exists() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::ModelNotFound,
                error_name: "MODEL_NOT_FOUND".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.model.not_found".to_string(),
                suggested_action_key: None,
            });
        }

        if let Some(ref checksum) = self.expected_sha256 {
            ModelRegistry::verify_weights_checksum(&self.model_path, checksum)
        } else {
            SiraResult::Success(true)
        }
    }
}

#[async_trait]
impl AiProvider for CandleLlmProvider {
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest {
            provider_id: "provider-candle-local-llm".to_string(),
            vendor_name: "Hugging Face Candle (Local Neural)".to_string(),
            supported_capabilities: vec![AICapability::TextGeneration],
            models: vec![AIModelInfo {
                model_id: self.model_id.clone(),
                display_name: self.display_name.clone(),
                context_window_tokens: 8192,
                vram_required_mb: 4096,
                checksum_sha256: self.expected_sha256.clone().unwrap_or_default(),
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
        // 1. Validate weights prior to execution
        match self.verify_weights() {
            SiraResult::Success(true) => {},
            SiraResult::Error(err) => return SiraResult::Error(err),
            _ => return SiraResult::Error(SiraError {
                code: SiraErrorCode::ModelNotFound,
                error_name: "MODEL_NOT_FOUND".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.model.not_found".to_string(),
                suggested_action_key: None,
            }),
        }

        // 2. Load GGUF file & device setup
        let mut file = match std::fs::File::open(&self.model_path) {
            Ok(f) => f,
            Err(e) => return SiraResult::Error(SiraError {
                code: SiraErrorCode::ModelUnreadable,
                error_name: "MODEL_UNREADABLE".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: format!("errors.model.unreadable: {}", e),
                suggested_action_key: None,
            }),
        };

        let content = match candle_core::quantized::gguf_file::Content::read(&mut file) {
            Ok(c) => c,
            Err(e) => return SiraResult::Error(SiraError {
                code: SiraErrorCode::InvalidModelFormat,
                error_name: "INVALID_MODEL_FORMAT".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: format!("errors.model.invalid_format: {}", e),
                suggested_action_key: None,
            }),
        };

        // Select Device: Metal on macOS, CPU on Windows baseline
        #[cfg(target_os = "macos")]
        let device = candle_core::Device::new_metal(0).unwrap_or(candle_core::Device::Cpu);
        #[cfg(not(target_os = "macos"))]
        let device = candle_core::Device::Cpu;

        let mut model = match candle_transformers::models::quantized_llama::ModelWeights::from_gguf(content, &mut file, &device) {
            Ok(m) => m,
            Err(e) => return SiraResult::Error(SiraError {
                code: SiraErrorCode::ModelInitializationFailed,
                error_name: "MODEL_INIT_FAILED".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: format!("errors.model.init_failed: {}", e),
                suggested_action_key: None,
            }),
        };

        // 3. Load Tokenizer & Execute Auto-Regressive Forward Generation Loop
        let tokenizer_path = self.model_path.with_file_name("tokenizer.json");
        let tokenizer = match tokenizers::Tokenizer::from_file(&tokenizer_path) {
            Ok(t) => t,
            Err(e) => return SiraResult::Error(SiraError {
                code: SiraErrorCode::InvalidModelFormat,
                error_name: "TOKENIZER_NOT_FOUND".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: format!("errors.tokenizer.missing: {}", e),
                suggested_action_key: None,
            }),
        };

        let tokens = match tokenizer.encode(request.prompt.as_str(), true) {
            Ok(t) => t.get_ids().to_vec(),
            Err(e) => return SiraResult::Error(SiraError {
                code: SiraErrorCode::InvalidModelFormat,
                error_name: "TOKENIZATION_FAILED".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: format!("errors.tokenization.failed: {}", e),
                suggested_action_key: None,
            }),
        };

        let mut logits_processor = candle_transformers::generation::LogitsProcessor::new(1337, Some(0.7), Some(0.9));
        let mut generated_tokens: Vec<u32> = Vec::new();
        let mut all_tokens = tokens.clone();
        let max_tokens = 64;

        for index in 0..max_tokens {
            let context = if index == 0 { &all_tokens[..] } else { &[all_tokens[all_tokens.len() - 1]] };
            let input = match candle_core::Tensor::new(context, &device) {
                Ok(t) => match t.unsqueeze(0) {
                    Ok(u) => u,
                    Err(_) => break,
                },
                Err(_) => break,
            };

            let logits = match model.forward(&input, all_tokens.len() - context.len()) {
                Ok(l) => l,
                Err(_) => break,
            };

            let logits = match logits.squeeze(0) {
                Ok(l) => match l.squeeze(0) {
                    Ok(l) => l,
                    Err(_) => break,
                },
                Err(_) => break,
            };

            let next_token = match logits_processor.sample(&logits) {
                Ok(t) => t,
                Err(_) => break,
            };

            generated_tokens.push(next_token);
            all_tokens.push(next_token);

            if next_token == 2 || next_token == 128001 || next_token == 128009 { // EOS tokens
                break;
            }
        }

        let generated_text = match tokenizer.decode(&generated_tokens, true) {
            Ok(text) => text,
            Err(e) => format!("[Decoding error: {}]", e),
        };

        let prompt_tokens = tokens.len();
        let completion_tokens = generated_tokens.len();

        SiraResult::Success(AIResponse {
            request_id: request.request_id,
            output_text: Some(generated_text),
            output_media_uri: None,
            usage: AIUsage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
                cost_usd: 0.0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_provider_manifest() {
        let provider = CandleLlmProvider::new(
            PathBuf::from("models/llm/test_model.gguf"),
            Some("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string()),
        );
        let manifest = provider.manifest();
        assert_eq!(manifest.provider_id, "provider-candle-local-llm");
        assert!(manifest.is_offline_capable);
        assert_eq!(manifest.supported_capabilities, vec![AICapability::TextGeneration]);
    }

    #[test]
    fn test_missing_model_verification() {
        let provider = CandleLlmProvider::new(
            PathBuf::from("non_existent_model_directory/model.gguf"),
            None,
        );
        match provider.verify_weights() {
            SiraResult::Error(err) => {
                assert_eq!(err.code, SiraErrorCode::ModelNotFound);
            },
            _ => panic!("Expected ModelNotFound error for missing model path"),
        }
    }

    #[tokio::test]
    async fn test_real_gguf_inference() {
        let model_path = PathBuf::from("models/llm/llama-3.2-3b-instruct.gguf");
        if !model_path.exists() {
            println!("GGUF model file not found at {:?}, skipping real inference test.", model_path);
            return;
        }

        let provider = CandleLlmProvider::new(model_path, None);
        let request = AIRequest {
            request_id: "real-inference-proof-001".to_string(),
            prompt: "Write one short sentence describing a peaceful sunrise over a Tamil village.".to_string(),
            media_url: None,
            options: std::collections::HashMap::new(),
        };

        let start = std::time::Instant::now();
        let res = provider.execute(request).await;
        let elapsed = start.elapsed();

        match res {
            SiraResult::Success(resp) => {
                println!("=== REAL GGUF INFERENCE PROOF ===");
                println!("Time Elapsed: {:?}", elapsed);
                println!("Prompt Tokens: {}", resp.usage.prompt_tokens);
                println!("Completion Tokens: {}", resp.usage.completion_tokens);
                println!("Generated Text:\n{}", resp.output_text.as_deref().unwrap_or(""));
                assert!(resp.output_text.is_some());
                assert!(resp.usage.completion_tokens > 0);
            },
            SiraResult::Error(err) => {
                panic!("Real GGUF inference failed: {:?}", err);
            }
        }
    }
}

