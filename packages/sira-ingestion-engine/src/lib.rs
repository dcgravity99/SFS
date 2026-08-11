/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod ingestion_manager;
pub mod format_decoder;
pub mod proxy_generator;
pub mod metadata_extractor;
pub mod color_convertor;

pub use ingestion_manager::ingest_media_file;
pub use format_decoder::detect_media_format;
pub use proxy_generator::generate_editing_proxy;
pub use metadata_extractor::extract_smpte_metadata;
pub use color_convertor::convert_to_aces_cg;
