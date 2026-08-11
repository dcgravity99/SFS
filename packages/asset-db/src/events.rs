/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::ids::AssetId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AssetMutationType {
    Created,
    Updated,
    StatusChanged,
    RelationshipLinked,
    SoftDeleted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssetMutationEvent {
    pub mutation_type: AssetMutationType,
    pub asset_id: AssetId,
    pub timestamp: String,
    pub payload_json: String,
}
