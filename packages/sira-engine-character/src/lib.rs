/* ============================================================================
 * Siragugal Film Studio — Module 43: AI Character Performance / Facial Animation Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod anchors;
pub mod consistency;
pub mod facial_anim;
pub mod lora_binding;
pub mod profile;

pub use consistency::verify_visual_consistency;
pub use facial_anim::*;
pub use profile::CharacterProfile;

pub struct CharacterEngine;

impl CharacterEngine {
    pub fn create_character(name: &str, role: &str) -> Result<CharacterProfile, String> {
        profile::create_character_profile(name, role)
    }

    pub fn verify_visual_consistency(
        target_embedding: &[f32],
        anchor_embedding: &[f32],
    ) -> Result<f32, String> {
        consistency::verify_visual_consistency(target_embedding, anchor_embedding)
    }
}
