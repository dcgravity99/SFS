/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FtsQueryFilter {
    pub search_term: String,
    pub asset_types: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub limit: usize,
}
