/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::lifecycle::AssetLifecycleStatus;
use serde::{Deserialize, Serialize};
use sira_types::ids::AssetId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniversalAssetRecord {
    pub asset_id: AssetId,
    pub asset_type: String,
    pub lifecycle_status: AssetLifecycleStatus,
    pub display_name: String,
    pub uri: String,
    pub mime_type: String,
    pub file_size_bytes: u64,
    pub checksum_sha256: String,
    pub version_branch: String,
    pub parent_asset_ids: Vec<String>,
    pub metadata_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipRecord {
    pub relationship_id: String,
    pub source_asset_id: AssetId,
    pub target_asset_id: AssetId,
    pub relationship_ontology: String, // 'CONTAINS', 'DEPENDS_ON', 'GENERATED_FROM', etc.
    pub metadata_json: Option<String>,
}
