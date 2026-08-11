/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use crate::errors::SiraError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SiraResult<T> {
    Success(T),
    PartialSuccess { data: T, warnings: Vec<SiraError> },
    Error(SiraError),
    Progress { progress: f32, stage: String },
    Cancelled { reason: String },
}
