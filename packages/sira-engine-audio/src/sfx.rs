/* ============================================================================
 * Siragugal Film Studio — Module 28: Special Effects (SFX) Sound Library Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SfxCategory {
    ActionImpact,
    FoleyFootsteps,
    CinematicsRiser,
    EnvironmentAmbience,
    SciFiWeapons,
    TamilMassStuntCue,
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfxSoundAsset {
    pub asset_id: String,
    pub name: String,
    pub category: SfxCategory,
    pub tags: Vec<String>,
    pub duration_seconds: f32,
    pub sample_rate_hz: u32,
    pub channel_count: u16,
    pub file_path: String,
    pub loudness_lufs: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SfxPlacementCue {
    pub cue_id: String,
    pub asset_id: String,
    pub start_timecode_seconds: f64,
    pub duration_seconds: f32,
    pub gain_db: f32,
    pub pan_lr: f32,
    pub fade_in_seconds: f32,
    pub fade_out_seconds: f32,
    pub is_looping: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SfxSearchQuery {
    pub category: Option<SfxCategory>,
    pub tag_filter: Option<String>,
    pub max_duration_seconds: Option<f32>,
    pub min_sample_rate_hz: Option<u32>,
}

#[derive(Default)]
pub struct SfxLibraryEngine {
    assets: Vec<SfxSoundAsset>,
}

impl SfxLibraryEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_sfx_asset(&mut self, asset: SfxSoundAsset) -> SiraResult<String> {
        let id = asset.asset_id.clone();
        if self.assets.iter().any(|a| a.asset_id == id) {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "DUPLICATE_SFX_ASSET_ID".to_string(),
                category: "AUDIO_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.sfx.duplicate_asset_id".to_string(),
                suggested_action_key: None,
            });
        }
        self.assets.push(asset);
        SiraResult::Success(id)
    }

    pub fn search_sfx_assets(&self, query: &SfxSearchQuery) -> SiraResult<Vec<SfxSoundAsset>> {
        let matched: Vec<SfxSoundAsset> = self
            .assets
            .iter()
            .filter(|a| {
                if let Some(cat) = &query.category {
                    if a.category != *cat {
                        return false;
                    }
                }
                if let Some(tag) = &query.tag_filter {
                    if !a.tags.iter().any(|t| t.to_lowercase().contains(&tag.to_lowercase())) {
                        return false;
                    }
                }
                if let Some(max_dur) = query.max_duration_seconds {
                    if a.duration_seconds > max_dur {
                        return false;
                    }
                }
                if let Some(min_rate) = query.min_sample_rate_hz {
                    if a.sample_rate_hz < min_rate {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        SiraResult::Success(matched)
    }

    pub fn create_placement_cue(
        &self,
        asset_id: &str,
        start_seconds: f64,
        gain_db: f32,
    ) -> SiraResult<SfxPlacementCue> {
        let asset = match self.assets.iter().find(|a| a.asset_id == asset_id) {
            Some(a) => a,
            None => {
                return SiraResult::Error(SiraError {
                    code: SiraErrorCode::UnknownSystemError,
                    error_name: "SFX_ASSET_NOT_FOUND".to_string(),
                    category: "AUDIO_ENGINE".to_string(),
                    severity: "ERROR".to_string(),
                    is_recoverable: false,
                    correlation_id: None,
                    job_id: None,
                    i18n_key: "errors.sfx.asset_not_found".to_string(),
                    suggested_action_key: None,
                });
            }
        };

        let cue = SfxPlacementCue {
            cue_id: format!("CUE-SFX-{}", uuid_or_timestamp(start_seconds)),
            asset_id: asset.asset_id.clone(),
            start_timecode_seconds: start_seconds,
            duration_seconds: asset.duration_seconds,
            gain_db: gain_db.clamp(-60.0, 12.0),
            pan_lr: 0.0,
            fade_in_seconds: 0.05,
            fade_out_seconds: 0.05,
            is_looping: false,
        };

        SiraResult::Success(cue)
    }

    pub fn validate_cue_placement(&self, cue: &SfxPlacementCue) -> SiraResult<bool> {
        if cue.gain_db < -60.0 || cue.gain_db > 12.0 {
            return SiraResult::Success(false);
        }
        if cue.pan_lr < -1.0 || cue.pan_lr > 1.0 {
            return SiraResult::Success(false);
        }
        if !self.assets.iter().any(|a| a.asset_id == cue.asset_id) {
            return SiraResult::Success(false);
        }
        SiraResult::Success(true)
    }
}

fn uuid_or_timestamp(start_seconds: f64) -> String {
    format!("{:08x}", (start_seconds * 1000.0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_28_sfx_library_lifecycle() {
        let mut engine = SfxLibraryEngine::new();

        let stunt_sfx = SfxSoundAsset {
            asset_id: "SFX_TAMIL_PUNCH_01".to_string(),
            name: "Tamil Cinema Fight Punch Impact".to_string(),
            category: SfxCategory::TamilMassStuntCue,
            tags: vec!["action".to_string(), "punch".to_string(), "mass".to_string()],
            duration_seconds: 1.2,
            sample_rate_hz: 48000,
            channel_count: 2,
            file_path: "assets/audio/sfx/tamil_punch.wav".to_string(),
            loudness_lufs: -14.0,
        };

        let riser_sfx = SfxSoundAsset {
            asset_id: "SFX_RISER_01".to_string(),
            name: "Cinematic Tension Riser".to_string(),
            category: SfxCategory::CinematicsRiser,
            tags: vec!["tension".to_string(), "riser".to_string()],
            duration_seconds: 5.0,
            sample_rate_hz: 48000,
            channel_count: 2,
            file_path: "assets/audio/sfx/riser_01.wav".to_string(),
            loudness_lufs: -16.0,
        };

        assert!(matches!(engine.register_sfx_asset(stunt_sfx.clone()), SiraResult::Success(_)));
        assert!(matches!(engine.register_sfx_asset(riser_sfx.clone()), SiraResult::Success(_)));

        // Test duplicate registration error
        assert!(matches!(engine.register_sfx_asset(stunt_sfx), SiraResult::Error(_)));

        // Test category search
        let query_stunt = SfxSearchQuery {
            category: Some(SfxCategory::TamilMassStuntCue),
            ..Default::default()
        };
        if let SiraResult::Success(results) = engine.search_sfx_assets(&query_stunt) {
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].asset_id, "SFX_TAMIL_PUNCH_01");
        } else {
            panic!("search_sfx_assets failed");
        }

        // Test duration filtering
        let query_short = SfxSearchQuery {
            max_duration_seconds: Some(2.0),
            ..Default::default()
        };
        if let SiraResult::Success(results) = engine.search_sfx_assets(&query_short) {
            assert_eq!(results.len(), 1);
            assert_eq!(results[0].asset_id, "SFX_TAMIL_PUNCH_01");
        } else {
            panic!("search_sfx_assets short query failed");
        }

        // Test placement cue creation
        let cue_res = engine.create_placement_cue("SFX_TAMIL_PUNCH_01", 10.5, -3.0);
        assert!(matches!(cue_res, SiraResult::Success(_)));

        if let SiraResult::Success(mut cue) = cue_res {
            assert_eq!(cue.asset_id, "SFX_TAMIL_PUNCH_01");
            assert_eq!(cue.gain_db, -3.0);

            // Test cue validation
            if let SiraResult::Success(is_valid) = engine.validate_cue_placement(&cue) {
                assert!(is_valid);
            } else {
                panic!("validate_cue_placement failed");
            }

            // Test invalid pan cue validation
            cue.pan_lr = 2.5; // Invalid pan (> 1.0)
            if let SiraResult::Success(is_valid) = engine.validate_cue_placement(&cue) {
                assert!(!is_valid);
            } else {
                panic!("validate_cue_placement invalid test failed");
            }
        } else {
            panic!("create_placement_cue failed");
        }
    }
}
