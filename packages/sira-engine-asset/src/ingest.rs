/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::checksum::ChecksumVerifier;
use crate::proxy::ProxyVideoGenerator;
use serde::{Deserialize, Serialize};
use sira_types::SiraResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestJobSpec {
    pub source_path: String,
    pub asset_type: String, // Video, Audio, Image, Mesh, Script
    pub create_proxy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestResult {
    pub asset_id: String,
    pub sha256_checksum: String,
    pub proxy_path: Option<String>,
    pub file_size_bytes: u64,
}

pub struct MediaIngestCoordinator;

impl MediaIngestCoordinator {
    pub fn ingest(spec: IngestJobSpec) -> SiraResult<IngestResult> {
        let asset_id = format!("ast-{}", spec.source_path.replace(['/', '\\', '.'], "_"));
        let checksum = ChecksumVerifier::compute_sha256(spec.source_path.as_bytes());

        let proxy_path = if spec.create_proxy {
            ProxyVideoGenerator::generate_proxy(&asset_id, "720p").ok()
        } else {
            None
        };

        SiraResult::Success(IngestResult {
            asset_id,
            sha256_checksum: checksum,
            proxy_path,
            file_size_bytes: spec.source_path.len() as u64 * 1024,
        })
    }
}
