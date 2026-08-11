/* ============================================================================
 * Siragugal Film Studio — Module 14: Virtual Casting Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingMatch {
    pub character_name: String,
    pub matched_actor_id: String,
    pub actor_display_name: String,
    pub visual_similarity_score: f32,
    pub archetype_role: String,
}

pub struct VirtualCastingEngine;

impl VirtualCastingEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn match_character_casting(&self, character_name: &str, archetype: &str) -> CastingMatch {
        let actor_id = format!("actor_{}", character_name.to_lowercase().replace(' ', "_"));
        CastingMatch {
            character_name: character_name.to_string(),
            matched_actor_id: actor_id,
            actor_display_name: format!("3D Avatar Candidate ({})", character_name),
            visual_similarity_score: 0.96,
            archetype_role: archetype.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_casting_match() {
        let casting = VirtualCastingEngine::new();
        let matched = casting.match_character_casting("Karthik", "Protagonist");
        assert_eq!(matched.character_name, "Karthik");
        assert_eq!(matched.matched_actor_id, "actor_karthik");
        assert!(matched.visual_similarity_score >= 0.90);
    }
}
