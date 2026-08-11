/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use sha2::{Sha256, Digest};

pub fn compute_cache_key(namespace: &str, inputs_json: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(namespace.as_bytes());
    hasher.update(inputs_json.as_bytes());
    format!("{:x}", hasher.finalize())
}
