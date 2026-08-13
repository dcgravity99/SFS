/* ============================================================================
 * Siragugal Film Studio — Module 53: Distribution Rights Validation Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod broadcast;
pub mod integration_audit;
pub mod ipc_verifier;
pub mod locale_auditor;
pub mod rights_validation;

pub use broadcast::*;
pub use integration_audit::run_full_system_integration_audit;
pub use rights_validation::*;
