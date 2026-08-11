/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::schema::SiraSettings;
use sira_types::SiraResult;

pub struct SettingsTransaction {
    pub transaction_id: String,
    pub draft: SiraSettings,
}

impl SettingsTransaction {
    pub fn begin(current: &SiraSettings) -> Self {
        Self {
            transaction_id: format!("tx-{:x}", sira_types::ids::ProjectId::new_v7().0.as_u128()),
            draft: current.clone(),
        }
    }

    pub fn commit(self) -> SiraResult<SiraSettings> {
        SiraResult::Success(self.draft)
    }
}
