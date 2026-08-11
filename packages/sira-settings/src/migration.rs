/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::schema::SiraSettings;
use sira_types::SiraResult;

pub fn migrate_settings_schema(settings: &mut SiraSettings) -> SiraResult<()> {
    if settings.version < 1 {
        settings.version = 1;
    }
    SiraResult::Success(())
}
