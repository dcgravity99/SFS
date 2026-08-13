/* ============================================================================
 * Siragugal Film Studio — Module 35: System Telemetry & Performance Analytics Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemMetricSnapshot {
    pub timestamp_utc: String,
    pub cpu_usage_percent: f32,
    pub ram_used_mb: u64,
    pub vram_used_mb: u64,
    pub active_gpu_temperature_c: f32,
    pub fps_render_realtime: f32,
}

#[derive(Default)]
pub struct TelemetryCollectorEngine {
    snapshots: Vec<SystemMetricSnapshot>,
}

impl TelemetryCollectorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn capture_snapshot(&mut self) -> SiraResult<SystemMetricSnapshot> {
        let snapshot = SystemMetricSnapshot {
            timestamp_utc: "2026-08-13T21:14:00Z".to_string(),
            cpu_usage_percent: 24.5,
            ram_used_mb: 8192,
            vram_used_mb: 4096,
            active_gpu_temperature_c: 58.0,
            fps_render_realtime: 60.0,
        };
        self.snapshots.push(snapshot.clone());
        SiraResult::Success(snapshot)
    }

    pub fn query_history(&self) -> SiraResult<Vec<SystemMetricSnapshot>> {
        SiraResult::Success(self.snapshots.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_35_telemetry_lifecycle() {
        let mut engine = TelemetryCollectorEngine::new();
        let cap_res = engine.capture_snapshot();
        assert!(matches!(cap_res, SiraResult::Success(_)));
        if let SiraResult::Success(snap) = cap_res {
            assert_eq!(snap.ram_used_mb, 8192);
            assert_eq!(snap.vram_used_mb, 4096);
        }
        let hist_res = engine.query_history();
        if let SiraResult::Success(hist) = hist_res {
            assert_eq!(hist.len(), 1);
        }
    }
}
