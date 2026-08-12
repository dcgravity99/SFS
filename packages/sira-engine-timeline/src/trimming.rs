/*
============================================================================

- Siragugal Film Studio
- Copyright (C) 2026 Siragugal Film Studio Contributors
- Licensed under Apache-2.0 or MIT.

============================================================================
*/

use crate::track::TimelineClip;
use sira_types::{SiraError, SiraErrorCode, SiraResult};

pub struct TimelineTrimmingCalculator;

impl TimelineTrimmingCalculator {
    pub fn razor_split(
        clip: &TimelineClip,
        split_frame: u64,
    ) -> SiraResult<(TimelineClip, TimelineClip)> {
        if split_frame <= clip.start_frame || split_frame >= clip.start_frame + clip.duration_frames
        {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::WorkflowDagCycleDetected,
                error_name: "INVALID_SPLIT_FRAME".to_string(),
                category: "TIMELINE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.timeline.invalid_split".to_string(),
                suggested_action_key: None,
            });
        }

        let first_duration = split_frame - clip.start_frame;

        let second_duration = clip.duration_frames - first_duration;

        let clip1 = TimelineClip {
            clip_id: format!("{}-a", clip.clip_id),
            asset_id: clip.asset_id.clone(),
            track_id: clip.track_id.clone(),
            in_frame: clip.in_frame,
            out_frame: clip.in_frame + first_duration,
            start_frame: clip.start_frame,
            duration_frames: first_duration,
        };

        let clip2 = TimelineClip {
            clip_id: format!("{}-b", clip.clip_id),
            asset_id: clip.asset_id.clone(),
            track_id: clip.track_id.clone(),
            in_frame: clip.in_frame + first_duration,
            out_frame: clip.out_frame,
            start_frame: split_frame,
            duration_frames: second_duration,
        };

        SiraResult::Success((clip1, clip2))
    }
}
