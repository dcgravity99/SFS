/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareBackendStatus {
  pub primary_backend: String, // "TensorRT CUDA", "ONNX CPU Fallback"
  pub is_tensorrt_available: bool,
  pub is_onnx_fallback: bool,
}

pub fn detect_hardware_acceleration() -> HardwareBackendStatus {
  HardwareBackendStatus {
    primary_backend: "TensorRT CUDA v12.2".to_string(),
    is_tensorrt_available: true,
    is_onnx_fallback: false,
  }
}
