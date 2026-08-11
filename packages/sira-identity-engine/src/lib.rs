/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod sso_manager;
pub mod oidc_connector;
pub mod saml_provider;
pub mod jwt_verifier;
pub mod session_auditor;

pub use sso_manager::authenticate_sso_provider;
pub use oidc_connector::exchange_oidc_code;
pub use saml_provider::validate_saml_assertion;
pub use jwt_verifier::verify_jwt_session_token;
pub use session_auditor::revoke_user_session;
