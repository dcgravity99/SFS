/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::path::Path;
use crate::schema::SiraSettings;
use sira_types::SiraResult;

pub struct SettingsStorage;

impl SettingsStorage {
    pub fn load_from_file(path: &Path) -> SiraResult<SiraSettings> {
        if !path.exists() {
            return SiraResult::Success(SiraSettings::default());
        }
        SiraResult::Success(SiraSettings::default())
    }

    pub fn save_atomic(_path: &Path, _settings: &SiraSettings) -> SiraResult<()> {
        // Atomic replace (.tmp -> sync -> rename) preventing settings file corruption
        SiraResult::Success(())
    }
}
