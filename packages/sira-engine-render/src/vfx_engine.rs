/* ============================================================================
 * Siragugal Film Studio — Module 23: Visual Effects (VFX) Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleSystemConfig {
    pub effect_type: String, // "AtmosphericMist", "SunlightBloom", "DustMotes"
    pub particle_count: u32,
    pub emission_rate: f32,
    pub lifetime_sec: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfxPassSpec {
    pub scene_vfx_id: String,
    pub particle_systems: Vec<ParticleSystemConfig>,
    pub enable_volumetric_mist: bool,
}

pub struct VfxEngine;

impl VfxEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_atmospheric_vfx(&self, vfx_id: &str) -> VfxPassSpec {
        let mist = ParticleSystemConfig {
            effect_type: "AtmosphericMist".to_string(),
            particle_count: 5000,
            emission_rate: 100.0,
            lifetime_sec: 4.0,
        };

        let bloom = ParticleSystemConfig {
            effect_type: "SunlightBloom".to_string(),
            particle_count: 500,
            emission_rate: 20.0,
            lifetime_sec: 2.5,
        };

        VfxPassSpec {
            scene_vfx_id: vfx_id.to_string(),
            particle_systems: vec![mist, bloom],
            enable_volumetric_mist: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfx_engine_setup() {
        let vfx = VfxEngine::new();
        let pass = vfx.setup_atmospheric_vfx("vfx_sunrise");
        assert_eq!(pass.scene_vfx_id, "vfx_sunrise");
        assert!(pass.enable_volumetric_mist);
        assert_eq!(pass.particle_systems.len(), 2);
    }
}
