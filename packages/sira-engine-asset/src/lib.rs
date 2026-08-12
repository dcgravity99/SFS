/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod checksum;
pub mod ingest;
pub mod proxy;
pub mod transcode;

pub use checksum::*;
pub use ingest::*;
pub use proxy::*;
pub use transcode::*;

use sira_types::SiraResult;

pub struct AssetPipelineEngine;

impl AssetPipelineEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn ingest_asset(&self, spec: IngestJobSpec) -> SiraResult<IngestResult> {
        MediaIngestCoordinator::ingest(spec)
    }

    pub fn generate_proxy(&self, asset_id: &str, target_resolution: &str) -> SiraResult<String> {
        ProxyVideoGenerator::generate_proxy(asset_id, target_resolution)
    }

    pub fn verify_checksum(&self, asset_id: &str, expected_hash: &str) -> SiraResult<bool> {
        ChecksumVerifier::verify_hash(asset_id.as_bytes(), expected_hash)
    }
}
