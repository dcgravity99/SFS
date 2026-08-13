/* ============================================================================
 * Siragugal Film Studio — Module 60: Master Studio Acceptance & Platform Certifier Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod ecosystem_auditor;
pub mod license_verifier;
pub mod locale_validator;
pub mod master_acceptance;
pub mod master_certifier;
pub mod master_dispatcher;
pub mod release_certifier;
pub mod tenant_security;

pub use ecosystem_auditor::audit_master_ecosystem;
pub use license_verifier::verify_license_compliance;
pub use locale_validator::validate_locale_integrity;
pub use master_acceptance::evaluate_master_acceptance;
pub use master_certifier::*;
pub use master_dispatcher::*;
pub use release_certifier::generate_master_certificate;
pub use tenant_security::*;
