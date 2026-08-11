/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::categories::CacheCategory;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheMetadataRecord {
    pub cache_key: String,
    pub category: CacheCategory,
    pub sha256: String,
    pub size_bytes: u64,
    pub artifact_path: PathBuf,
    pub created_at: String,
    pub last_accessed_at: String,
    pub access_count: u32,
    pub owner_module: String,
    pub workflow_id: Option<String>,
}

pub struct CacheIndexDb;

impl CacheIndexDb {
    pub const CREATE_INDEX_SQL: &'static str = r#"
CREATE TABLE IF NOT EXISTS cache_index (
    cache_key TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_accessed_at TEXT NOT NULL,
    access_count INTEGER DEFAULT 1,
    owner_module TEXT NOT NULL,
    workflow_id TEXT
);
CREATE INDEX IF NOT EXISTS idx_cache_category ON cache_index(category);
"#;
}
