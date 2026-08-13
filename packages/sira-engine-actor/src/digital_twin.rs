/* ============================================================================
 * Siragugal Film Studio — Module 50: Digital Twin Actor Control Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DigitalTwinState {
    pub twin_id: String,
    pub actor_name: String,
    pub latency_ms: f32,
    pub active_animation_clip: String,
}

#[derive(Default)]
pub struct DigitalTwinEngine;

impl DigitalTwinEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_digital_twin(&self, actor_name: &str) -> SiraResult<DigitalTwinState> {
        if actor_name.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_ACTOR_NAME".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.digital_twin.empty_name".to_string(),
                suggested_action_key: None,
            });
        }

        if actor_name.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_ACTOR_NAME_PATH".to_string(),
                category: "ACTOR_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.digital_twin.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let state = DigitalTwinState {
            twin_id: format!("TWIN-{}", actor_name.to_uppercase()),
            actor_name: actor_name.to_string(),
            latency_ms: 12.4,
            active_animation_clip: "IDLE_NEUTRAL".to_string(),
        };

        SiraResult::Success(state)
    }

    pub fn sync_live_state(&self, twin_id: &str, _pose_data: &[f32]) -> SiraResult<bool> {
        if twin_id.is_empty() {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_50_digital_twin_lifecycle() {
        let engine = DigitalTwinEngine::new();
        let twin_res = engine.register_digital_twin("Superstar_Rajini");
        assert!(matches!(twin_res, SiraResult::Success(_)));

        if let SiraResult::Success(state) = twin_res {
            assert_eq!(state.twin_id, "TWIN-SUPERSTAR_RAJINI");
            assert_eq!(state.active_animation_clip, "IDLE_NEUTRAL");
            let sync_res = engine.sync_live_state(&state.twin_id, &[0.0, 1.0, 0.0]);
            assert!(matches!(sync_res, SiraResult::Success(true)));
        }

        // Test empty name rejection
        assert!(matches!(engine.register_digital_twin(""), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.register_digital_twin("Actor/../traversed"), SiraResult::Error(_)));
    }
}
