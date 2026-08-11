/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersions {
    pub manifest: u32,
    pub database: u32,
    pub asset_index: u32,
    pub workflow_graph: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageIntegrity {
    pub db_sha256: String,
    pub workflow_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SfspManifest {
    pub format_version: String,
    pub schema_versions: SchemaVersions,
    pub project_id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub app_version_created: String,
    pub app_version_min: String,
    pub integrity: PackageIntegrity,
}

impl Default for SfspManifest {
    fn default() -> Self {
        Self {
            format_version: "1.0.0".to_string(),
            schema_versions: SchemaVersions {
                manifest: 1,
                database: 1,
                asset_index: 1,
                workflow_graph: 1,
            },
            project_id: sira_types::ids::ProjectId::new_v7().0.to_string(),
            title: "Untitled Project".to_string(),
            created_at: "2026-08-03T10:10:00.000Z".to_string(),
            updated_at: "2026-08-03T10:10:00.000Z".to_string(),
            app_version_created: "v0.1.0-alpha".to_string(),
            app_version_min: "v0.1.0-alpha".to_string(),
            integrity: PackageIntegrity {
                db_sha256: "".to_string(),
                workflow_sha256: "".to_string(),
            },
        }
    }
}
