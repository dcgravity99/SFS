/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetLifecycleStatus {
    Draft,
    Generated,
    Imported,
    Edited,
    Approved,
    Published,
    Archived,
    SoftDeleted,
}

impl AssetLifecycleStatus {
    pub fn can_transition_to(&self, target: &AssetLifecycleStatus) -> bool {
        match (self, target) {
            (AssetLifecycleStatus::SoftDeleted, _) => false, // Terminal unless restored
            (_, AssetLifecycleStatus::SoftDeleted) => true,
            (AssetLifecycleStatus::Draft, AssetLifecycleStatus::Generated) => true,
            (AssetLifecycleStatus::Generated, AssetLifecycleStatus::Edited) => true,
            (AssetLifecycleStatus::Edited, AssetLifecycleStatus::Approved) => true,
            (AssetLifecycleStatus::Approved, AssetLifecycleStatus::Published) => true,
            (AssetLifecycleStatus::Published, AssetLifecycleStatus::Archived) => true,
            _ => true,
        }
    }
}
