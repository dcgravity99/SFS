/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::timeline::NleTimeline;
use sira_types::SiraResult;

pub struct TimelineExporter;

impl TimelineExporter {
    pub fn export_to_json(timeline: &NleTimeline) -> SiraResult<String> {
        let json = serde_json::to_string_pretty(timeline).ok();
        SiraResult::Success(json.unwrap_or_else(|| "{}".to_string()))
    }
}
