/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sha2::{Digest, Sha256};
use sira_types::SiraResult;

pub struct ChecksumVerifier;

impl ChecksumVerifier {
    pub fn compute_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_hash(data: &[u8], expected_hash: &str) -> SiraResult<bool> {
        let actual = Self::compute_sha256(data);
        SiraResult::Success(actual.eq_ignore_ascii_case(expected_hash))
    }
}
