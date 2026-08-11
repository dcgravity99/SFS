/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::shot_plan::ShotPlan;
use sira_types::SiraResult;

pub struct EmotionalPacingEvaluator;

impl EmotionalPacingEvaluator {
    pub fn evaluate_pacing(shots: &[ShotPlan]) -> SiraResult<f32> {
        if shots.is_empty() {
            return SiraResult::Success(0.5);
        }

        let total_duration: f32 = shots.iter().map(|s| s.duration_seconds).sum();
        let avg_duration = total_duration / shots.len() as f32;

        // Shorter average shot duration indicates higher emotional intensity
        let intensity = (10.0 - avg_duration).clamp(0.0, 10.0) / 10.0;
        SiraResult::Success(intensity)
    }
}
