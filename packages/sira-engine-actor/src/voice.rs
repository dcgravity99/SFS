/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sira_types::SiraResult;

pub struct VoiceProviderAbstraction;

impl VoiceProviderAbstraction {
    pub fn bind_voice(actor_id: &str, voice_model_id: &str) -> SiraResult<()> {
        let _ = actor_id;
        let _ = voice_model_id;
        SiraResult::Success(())
    }
}
