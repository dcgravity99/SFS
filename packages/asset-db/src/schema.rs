/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub const CREATE_ASSETS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS assets (
    asset_id TEXT PRIMARY KEY,
    asset_type TEXT NOT NULL,
    lifecycle_status TEXT NOT NULL,
    display_name TEXT NOT NULL,
    uri TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    version_branch TEXT DEFAULT 'main',
    parent_asset_ids TEXT,
    metadata_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_assets_type_status ON assets(asset_type, lifecycle_status);
CREATE INDEX IF NOT EXISTS idx_assets_checksum ON assets(checksum_sha256);
"#;

pub const CREATE_RELATIONSHIPS_TABLE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS asset_relationships (
    relationship_id TEXT PRIMARY KEY,
    source_asset_id TEXT NOT NULL,
    target_asset_id TEXT NOT NULL,
    relationship_ontology TEXT NOT NULL,
    metadata_json TEXT,
    FOREIGN KEY(source_asset_id) REFERENCES assets(asset_id) ON DELETE CASCADE,
    FOREIGN KEY(target_asset_id) REFERENCES assets(asset_id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_rel_source_ontology ON asset_relationships(source_asset_id, relationship_ontology);
"#;
