/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum SiraErrorCode {
    UnknownSystemError = 1000,
    WorkspaceInitializationFailed = 1001,
    ConfigSchemaInvalid = 1002,
    HalDeviceNotFound = 2000,
    CudaVramAllocationOom = 2015,
    ModelNotFound = 3000,
    ModelUnreadable = 3009,
    InvalidModelFormat = 3010,
    ModelInitializationFailed = 3011,
    ModelChecksumVerificationFailed = 3008,
    SfspManifestCorrupted = 4002,
    WorkflowDagCycleDetected = 5012,
    PluginPermissionDenied = 6004,
    RenderCheckpointResumeFailed = 7009,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Error)]
#[error("[SIRA-{code:?}] {error_name}: {i18n_key}")]
pub struct SiraError {
    pub code: SiraErrorCode,
    pub error_name: String,
    pub category: String,
    pub severity: String,
    pub is_recoverable: bool,
    pub correlation_id: Option<String>,
    pub job_id: Option<String>,
    pub i18n_key: String,
    pub suggested_action_key: Option<String>,
}
