/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod inference_benchmark;
pub mod model_cache;
pub mod model_quantizer;
pub mod tensorrt_backend;
pub mod vram_optimizer;

pub use inference_benchmark::benchmark_neural_inference;
pub use model_cache::clear_model_cache;
pub use model_quantizer::optimize_model_precision;
pub use tensorrt_backend::detect_hardware_acceleration;
pub use vram_optimizer::optimize_vram_tiling;
