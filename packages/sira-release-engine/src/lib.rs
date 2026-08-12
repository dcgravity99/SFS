/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod integration_audit;
pub mod ipc_verifier;
pub mod locale_auditor;

pub use integration_audit::run_full_system_integration_audit;
