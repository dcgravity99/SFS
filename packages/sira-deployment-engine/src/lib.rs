/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod installer_builder;
pub mod signing_verifier;
pub mod update_manager;
pub mod deployment_manifest;

pub use installer_builder::build_production_release_package;
pub use signing_verifier::verify_code_signature;
pub use update_manager::generate_auto_update_manifest;
pub use deployment_manifest::generate_deployment_manifest;
