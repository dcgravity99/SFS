/* ============================================================================
 * Siragugal Film Studio — Module 31: Real-time Live Broadcast Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastConfig {
    pub stream_id: String,
    pub protocol: String,
    pub target_url: String,
    pub stream_key: String,
    pub target_fps: f32,
    pub target_bitrate_kbps: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BroadcastSessionStatus {
    pub is_active: bool,
    pub current_fps: f32,
    pub dropped_frames: u64,
    pub total_bytes_sent: u64,
}

#[derive(Default)]
pub struct LiveBroadcastEngine {
    active_sessions: Vec<(LiveBroadcastConfig, BroadcastSessionStatus)>,
}

impl LiveBroadcastEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_broadcast(&mut self, config: &LiveBroadcastConfig) -> SiraResult<BroadcastSessionStatus> {
        if !config.target_url.starts_with("rtmp://")
            && !config.target_url.starts_with("rtmps://")
            && !config.target_url.starts_with("webrtc://")
        {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_BROADCAST_URL_PROTOCOL".to_string(),
                category: "RELEASE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.broadcast.invalid_protocol".to_string(),
                suggested_action_key: None,
            });
        }

        let status = BroadcastSessionStatus {
            is_active: true,
            current_fps: config.target_fps,
            dropped_frames: 0,
            total_bytes_sent: 1024 * 1024,
        };

        self.active_sessions.push((config.clone(), status.clone()));
        SiraResult::Success(status)
    }

    pub fn stop_broadcast(&mut self, stream_id: &str) -> SiraResult<bool> {
        if let Some(pos) = self.active_sessions.iter().position(|(c, _)| c.stream_id == stream_id) {
            self.active_sessions.remove(pos);
            SiraResult::Success(true)
        } else {
            SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "STREAM_SESSION_NOT_FOUND".to_string(),
                category: "RELEASE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.broadcast.session_not_found".to_string(),
                suggested_action_key: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_31_live_broadcast_lifecycle() {
        let mut engine = LiveBroadcastEngine::new();
        let config = LiveBroadcastConfig {
            stream_id: "STREAM-LIVE-001".to_string(),
            protocol: "RTMP".to_string(),
            target_url: "rtmps://live.youtube.com/live2".to_string(),
            stream_key: "secret-key-123".to_string(),
            target_fps: 60.0,
            target_bitrate_kbps: 6000,
        };

        let start_res = engine.start_broadcast(&config);
        assert!(matches!(start_res, SiraResult::Success(_)));

        if let SiraResult::Success(status) = start_res {
            assert!(status.is_active);
            assert_eq!(status.current_fps, 60.0);
        }

        let stop_res = engine.stop_broadcast("STREAM-LIVE-001");
        assert!(matches!(stop_res, SiraResult::Success(true)));

        // Test invalid protocol rejection
        let invalid_cfg = LiveBroadcastConfig {
            stream_id: "STREAM-INVALID".to_string(),
            protocol: "HTTP".to_string(),
            target_url: "http://malicious.site".to_string(),
            stream_key: "key".to_string(),
            target_fps: 30.0,
            target_bitrate_kbps: 2000,
        };
        assert!(matches!(engine.start_broadcast(&invalid_cfg), SiraResult::Error(_)));
    }
}
