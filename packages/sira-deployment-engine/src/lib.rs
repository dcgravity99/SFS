/* ============================================================================
 * Siragugal Film Studio — Module 32: Automated Render Farm Dispatcher
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod deployment_manifest;
pub mod installer_builder;
pub mod render_farm;
pub mod signing_verifier;
pub mod update_manager;

pub use deployment_manifest::generate_deployment_manifest;
pub use installer_builder::build_production_release_package;
pub use render_farm::*;
pub use signing_verifier::verify_code_signature;
pub use update_manager::generate_auto_update_manifest;
