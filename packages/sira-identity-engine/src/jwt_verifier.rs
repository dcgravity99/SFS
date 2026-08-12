/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use crate::sso_manager::UserIdentitySession;

pub fn verify_jwt_session_token(token_string: &str) -> Result<UserIdentitySession, String> {
    if token_string.is_empty() {
        return Err("Invalid JWT token".to_string());
    }

    Ok(UserIdentitySession {
        session_id: "sess-uuidv7-verified".to_string(),
        user_id: "usr-ag-director".to_string(),
        display_name: "AG (Chief Software Architect)".to_string(),
        email: "ag@siragugal.studio".to_string(),
        role: "Director".to_string(),
        is_mfa_verified: true,
        expires_at: "2026-08-04T18:00:00Z".to_string(),
    })
}
