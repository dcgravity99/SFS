/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use async_trait::async_trait;
use sira_core::capabilities::AICapability;
use sira_types::{SiraError, SiraErrorCode, SiraResult};
use std::path::{Path, PathBuf};

use crate::contracts::{AIRequest, AIResponse, AIUsage};
use crate::manifest::{AIModelInfo, ProviderManifest};
use crate::model_registry::ModelRegistry;
use crate::provider_trait::AiProvider;

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

    fn model_error(
        code: SiraErrorCode,
        error_name: &str,
        i18n_key: String,
    ) -> SiraResult<AIResponse> {
        SiraResult::Error(SiraError {
            code,
            error_name: error_name.to_string(),
            category: "AI_PROVIDER".to_string(),
            severity: "ERROR".to_string(),
            is_recoverable: false,
            correlation_id: None,
            job_id: None,
            i18n_key,
            suggested_action_key: None,
        })
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
        // ---------------------------------------------------------------------
        // 1. Validate model weights
        // ---------------------------------------------------------------------
        match self.verify_weights() {
            SiraResult::Success(true) => {}

            SiraResult::Success(false) => {
                return Self::model_error(
                    SiraErrorCode::ModelNotFound,
                    "MODEL_NOT_FOUND",
                    "errors.model.not_found".to_string(),
                );
            }

            SiraResult::Error(err) => {
                return SiraResult::Error(err);
            }

            SiraResult::PartialSuccess {
                data: _,
                warnings: _,
            } => {
                return Self::model_error(
                    SiraErrorCode::ModelNotFound,
                    "MODEL_VERIFICATION_PARTIAL",
                    "errors.model.verification_partial".to_string(),
                );
            }

            SiraResult::Progress {
                progress: _,
                stage: _,
            } => {
                return Self::model_error(
                    SiraErrorCode::ModelNotFound,
                    "MODEL_VERIFICATION_IN_PROGRESS",
                    "errors.model.verification_in_progress".to_string(),
                );
            }

            SiraResult::Cancelled { reason } => {
                return Self::model_error(
                    SiraErrorCode::ModelNotFound,
                    "MODEL_VERIFICATION_CANCELLED",
                    format!("errors.model.verification_cancelled: {}", reason),
                );
            }
        }

        // ---------------------------------------------------------------------
        // 2. Open GGUF model
        // ---------------------------------------------------------------------
        let mut file = match std::fs::File::open(&self.model_path) {
            Ok(file) => file,
            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::ModelUnreadable,
                    "MODEL_UNREADABLE",
                    format!("errors.model.unreadable: {}", error),
                );
            }
        };

        // ---------------------------------------------------------------------
        // 3. Read GGUF metadata/content
        // ---------------------------------------------------------------------
        let content = match candle_core::quantized::gguf_file::Content::read(&mut file) {
            Ok(content) => content,
            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::InvalidModelFormat,
                    "INVALID_MODEL_FORMAT",
                    format!("errors.model.invalid_format: {}", error),
                );
            }
        };

        // ---------------------------------------------------------------------
        // 4. Select compute device
        // ---------------------------------------------------------------------
        #[cfg(target_os = "macos")]
        let device = candle_core::Device::new_metal(0).unwrap_or(candle_core::Device::Cpu);

        #[cfg(not(target_os = "macos"))]
        let device = candle_core::Device::Cpu;

        // ---------------------------------------------------------------------
        // 5. Load quantized Llama model
        // ---------------------------------------------------------------------
        let mut model = match candle_transformers::models::quantized_llama::ModelWeights::from_gguf(
            content, &mut file, &device,
        ) {
            Ok(model) => model,
            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::ModelInitializationFailed,
                    "MODEL_INIT_FAILED",
                    format!("errors.model.init_failed: {}", error),
                );
            }
        };

        // ---------------------------------------------------------------------
        // 6. Locate tokenizer
        // ---------------------------------------------------------------------
        let tokenizer_path = self.model_path.with_file_name("tokenizer.json");

        let tokenizer = match tokenizers::Tokenizer::from_file(&tokenizer_path) {
            Ok(tokenizer) => tokenizer,
            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::InvalidModelFormat,
                    "TOKENIZER_NOT_FOUND",
                    format!("errors.tokenizer.missing: {}", error),
                );
            }
        };

        // ---------------------------------------------------------------------
        // 7. Tokenize prompt
        // ---------------------------------------------------------------------
        let tokens = match tokenizer.encode(request.prompt.as_str(), true) {
            Ok(encoded) => encoded.get_ids().to_vec(),

            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::InvalidModelFormat,
                    "TOKENIZATION_FAILED",
                    format!("errors.tokenization.failed: {}", error),
                );
            }
        };

        if tokens.is_empty() {
            return Self::model_error(
                SiraErrorCode::InvalidModelFormat,
                "EMPTY_TOKENIZATION",
                "errors.tokenization.empty".to_string(),
            );
        }

        // ---------------------------------------------------------------------
        // 8. Generation configuration
        //
        // parameters_json is intentionally parsed defensively.
        // The provider remains usable if the caller supplies "{}" or invalid
        // optional parameters.
        // ---------------------------------------------------------------------
        let mut max_tokens: usize = 64;
        let mut temperature: f64 = 0.7;
        let mut top_p: f64 = 0.9;
        let mut seed: u64 = 1337;

        if let Ok(parameters) = serde_json::from_str::<serde_json::Value>(&request.parameters_json)
        {
            if let Some(value) = parameters.get("max_tokens").and_then(|v| v.as_u64()) {
                max_tokens = value.clamp(1, 4096) as usize;
            }

            if let Some(value) = parameters.get("temperature").and_then(|v| v.as_f64()) {
                temperature = value.clamp(0.0, 2.0);
            }

            if let Some(value) = parameters.get("top_p").and_then(|v| v.as_f64()) {
                top_p = value.clamp(0.0, 1.0);
            }

            if let Some(value) = parameters.get("seed").and_then(|v| v.as_u64()) {
                seed = value;
            }
        }

        // ---------------------------------------------------------------------
        // 9. Autoregressive generation
        // ---------------------------------------------------------------------
        let mut logits_processor = candle_transformers::generation::LogitsProcessor::new(
            seed,
            Some(temperature),
            Some(top_p),
        );

        let mut generated_tokens: Vec<u32> = Vec::new();
        let mut all_tokens = tokens.clone();

        for index in 0..max_tokens {
            let context: Vec<u32> = if index == 0 {
                all_tokens.clone()
            } else {
                vec![*all_tokens.last().unwrap_or(&tokens[0])]
            };

            let input = match candle_core::Tensor::new(context.as_slice(), &device) {
                Ok(tensor) => match tensor.unsqueeze(0) {
                    Ok(tensor) => tensor,
                    Err(error) => {
                        return Self::model_error(
                            SiraErrorCode::ModelInitializationFailed,
                            "INPUT_TENSOR_FAILED",
                            format!("errors.inference.input_tensor: {}", error),
                        );
                    }
                },

                Err(error) => {
                    return Self::model_error(
                        SiraErrorCode::ModelInitializationFailed,
                        "INPUT_TENSOR_FAILED",
                        format!("errors.inference.input_tensor: {}", error),
                    );
                }
            };

            let logits = match model.forward(&input, all_tokens.len().saturating_sub(context.len()))
            {
                Ok(logits) => logits,

                Err(error) => {
                    return Self::model_error(
                        SiraErrorCode::ModelInitializationFailed,
                        "MODEL_FORWARD_FAILED",
                        format!("errors.inference.forward_failed: {}", error),
                    );
                }
            };

            let logits = match logits.squeeze(0) {
                Ok(logits) => match logits.squeeze(0) {
                    Ok(logits) => logits,
                    Err(error) => {
                        return Self::model_error(
                            SiraErrorCode::ModelInitializationFailed,
                            "LOGITS_SHAPE_FAILED",
                            format!("errors.inference.logits_shape: {}", error),
                        );
                    }
                },

                Err(error) => {
                    return Self::model_error(
                        SiraErrorCode::ModelInitializationFailed,
                        "LOGITS_SHAPE_FAILED",
                        format!("errors.inference.logits_shape: {}", error),
                    );
                }
            };

            let next_token = match logits_processor.sample(&logits) {
                Ok(token) => token,

                Err(error) => {
                    return Self::model_error(
                        SiraErrorCode::ModelInitializationFailed,
                        "TOKEN_SAMPLING_FAILED",
                        format!("errors.inference.token_sampling: {}", error),
                    );
                }
            };

            generated_tokens.push(next_token);
            all_tokens.push(next_token);

            // Common EOS identifiers used by Llama-family tokenizers.
            if next_token == 2 || next_token == 128001 || next_token == 128009 {
                break;
            }
        }

        // ---------------------------------------------------------------------
        // 10. Decode generated tokens
        // ---------------------------------------------------------------------
        let generated_text = match tokenizer.decode(&generated_tokens, true) {
            Ok(text) => text,

            Err(error) => {
                return Self::model_error(
                    SiraErrorCode::InvalidModelFormat,
                    "DECODING_FAILED",
                    format!("errors.tokenization.decoding_failed: {}", error),
                );
            }
        };

        // ---------------------------------------------------------------------
        // 11. Build response
        // ---------------------------------------------------------------------
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

        assert_eq!(
            manifest.supported_capabilities,
            vec![AICapability::TextGeneration]
        );
    }

    #[test]
    fn test_missing_model_verification() {
        let provider = CandleLlmProvider::new(
            PathBuf::from("non_existent_model_directory/model.gguf"),
            None,
        );

        match provider.verify_weights() {
            SiraResult::Error(error) => {
                assert_eq!(error.code, SiraErrorCode::ModelNotFound);
            }

            _ => {
                panic!("Expected ModelNotFound error for missing model path");
            }
        }
    }

    #[tokio::test]
    async fn test_real_gguf_inference() {
        let model_path = PathBuf::from("models/llm/llama-3.2-3b-instruct.gguf");

        if !model_path.exists() {
            println!(
                "GGUF model file not found at {:?}, skipping real inference test.",
                model_path
            );
            return;
        }

        let provider = CandleLlmProvider::new(model_path, None);

        let request = AIRequest {
            request_id: "real-inference-proof-001".to_string(),

            capability: AICapability::TextGeneration,

            model_id: provider.model_id.clone(),

            prompt: "Write one short sentence describing a peaceful sunrise over a Tamil village."
                .to_string(),

            parameters_json: serde_json::json!({
                "max_tokens": 64,
                "temperature": 0.7,
                "top_p": 0.9,
                "seed": 1337
            })
            .to_string(),
        };

        let start = std::time::Instant::now();

        let result = provider.execute(request).await;

        let elapsed = start.elapsed();

        match result {
            SiraResult::Success(response) => {
                println!("=== REAL GGUF INFERENCE PROOF ===");
                println!("Time Elapsed: {:?}", elapsed);
                println!("Prompt Tokens: {}", response.usage.prompt_tokens);
                println!("Completion Tokens: {}", response.usage.completion_tokens);
                println!(
                    "Generated Text:\n{}",
                    response.output_text.as_deref().unwrap_or("")
                );

                assert!(response.output_text.is_some());
                assert!(response.usage.completion_tokens > 0);
            }

            SiraResult::PartialSuccess {
                data: response,
                warnings,
            } => {
                println!(
                    "Inference returned partial success with {} warning(s).",
                    warnings.len()
                );

                println!(
                    "Generated Text:\n{}",
                    response.output_text.as_deref().unwrap_or("")
                );

                assert!(response.output_text.is_some());
                assert!(response.usage.completion_tokens > 0);
            }

            SiraResult::Error(error) => {
                panic!("Real GGUF inference failed: {:?}", error);
            }

            SiraResult::Progress { progress, stage } => {
                panic!(
                    "Inference unexpectedly returned progress state: {}% - {}",
                    progress, stage
                );
            }

            SiraResult::Cancelled { reason } => {
                panic!("Real GGUF inference was cancelled: {}", reason);
            }
        }
    }
}
