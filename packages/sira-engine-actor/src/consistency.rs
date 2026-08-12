/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;

pub struct VoiceConsistencyValidator;

impl VoiceConsistencyValidator {
    pub fn verify_voice_consistency(
        target_embedding: &[f32],
        anchor_embedding: &[f32],
    ) -> SiraResult<f32> {
        if target_embedding.len() != anchor_embedding.len() || target_embedding.is_empty() {
            return SiraResult::Success(0.0);
        }

        let mut dot_product = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for (a, b) in target_embedding.iter().zip(anchor_embedding.iter()) {
            dot_product += a * b;
            norm_a += a * a;
            norm_b += b * b;
        }

        if norm_a == 0.0 || norm_b == 0.0 {
            SiraResult::Success(0.0)
        } else {
            let cosine = dot_product / (norm_a.sqrt() * norm_b.sqrt());
            SiraResult::Success(cosine.clamp(0.0, 1.0))
        }
    }
}
