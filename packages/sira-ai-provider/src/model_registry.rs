/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::path::Path;
use sha2::{Sha256, Digest};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

pub struct ModelRegistry;

impl ModelRegistry {
    pub fn verify_weights_checksum(file_path: &Path, expected_sha256: &str) -> SiraResult<bool> {
        if !file_path.exists() {
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

        let data = std::fs::read(file_path).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let computed = format!("{:x}", hasher.finalize());

        if computed != expected_sha256 {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::ModelChecksumVerificationFailed,
                error_name: "MODEL_CHECKSUM_VERIFICATION_FAILED".to_string(),
                category: "AI_PROVIDER".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.model.checksum_failed".to_string(),
                suggested_action_key: None,
            });
        }

        SiraResult::Success(true)
    }
}
