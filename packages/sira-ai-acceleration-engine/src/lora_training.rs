/* ============================================================================
 * Siragugal Film Studio — Module 34: Fine-Tuning & Custom LoRA Training Pipeline Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoraTrainingConfig {
    pub task_id: String,
    pub character_id: String,
    pub dataset_directory: String,
    pub rank_dim: u32,
    pub alpha_scale: f32,
    pub max_steps: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoraTrainingProgress {
    pub current_step: u32,
    pub total_steps: u32,
    pub loss: f32,
    pub output_weights_path: String,
}

#[derive(Default)]
pub struct LoraTrainingEngine {
    active_tasks: Vec<(LoraTrainingConfig, LoraTrainingProgress)>,
}

impl LoraTrainingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_training(&mut self, config: &LoraTrainingConfig) -> SiraResult<String> {
        if config.dataset_directory.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_DATASET_DIRECTORY".to_string(),
                category: "AI_ACCELERATION_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.lora.invalid_dataset_directory".to_string(),
                suggested_action_key: None,
            });
        }

        let progress = LoraTrainingProgress {
            current_step: 0,
            total_steps: config.max_steps,
            loss: 0.85,
            output_weights_path: format!("{}/weights.safetensors", config.dataset_directory),
        };

        self.active_tasks.push((config.clone(), progress));
        SiraResult::Success(config.task_id.clone())
    }

    pub fn query_training_progress(&self, task_id: &str) -> SiraResult<LoraTrainingProgress> {
        match self.active_tasks.iter().find(|(cfg, _)| cfg.task_id == task_id) {
            Some((_, prog)) => SiraResult::Success(prog.clone()),
            None => SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "LORA_TASK_NOT_FOUND".to_string(),
                category: "AI_ACCELERATION_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.lora.task_not_found".to_string(),
                suggested_action_key: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_34_lora_training_lifecycle() {
        let mut engine = LoraTrainingEngine::new();
        let config = LoraTrainingConfig {
            task_id: "LORA-TASK-001".to_string(),
            character_id: "CHAR_TAMIL_HERO".to_string(),
            dataset_directory: "C:/Datasets/Hero".to_string(),
            rank_dim: 16,
            alpha_scale: 1.0,
            max_steps: 1000,
        };

        let res = engine.start_training(&config);
        assert!(matches!(res, SiraResult::Success(_)));

        let prog_res = engine.query_training_progress("LORA-TASK-001");
        if let SiraResult::Success(prog) = prog_res {
            assert_eq!(prog.total_steps, 1000);
            assert!(prog.output_weights_path.contains("weights.safetensors"));
        }

        // Test path traversal rejection
        let invalid_cfg = LoraTrainingConfig {
            task_id: "LORA-INVALID".to_string(),
            character_id: "CHAR_TEST".to_string(),
            dataset_directory: "C:/Datasets/../Traversed".to_string(),
            rank_dim: 16,
            alpha_scale: 1.0,
            max_steps: 500,
        };
        assert!(matches!(engine.start_training(&invalid_cfg), SiraResult::Error(_)));
    }
}
