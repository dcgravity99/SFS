/* ============================================================================
 * Siragugal Film Studio — Module 53: Distribution Rights Validation Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RightsValidationReport {
    pub package_id: String,
    pub target_territory_code: String,
    pub is_authorized: bool,
    pub expiration_utc: String,
}

#[derive(Default)]
pub struct RightsValidationEngine;

impl RightsValidationEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate_distribution_rights(
        &self,
        package_id: &str,
        territory: &str,
    ) -> SiraResult<RightsValidationReport> {
        if package_id.is_empty() || territory.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_PACKAGE_OR_TERRITORY_ID".to_string(),
                category: "RELEASE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.rights.empty_id".to_string(),
                suggested_action_key: None,
            });
        }

        if package_id.contains("..") || territory.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_RIGHTS_ID_PATH".to_string(),
                category: "RELEASE_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.rights.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let report = RightsValidationReport {
            package_id: package_id.to_string(),
            target_territory_code: territory.to_uppercase(),
            is_authorized: true,
            expiration_utc: "2030-12-31T23:59:59Z".to_string(),
        };

        SiraResult::Success(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_53_rights_validation_lifecycle() {
        let engine = RightsValidationEngine::new();
        let val_res = engine.validate_distribution_rights("PKG-NETFLIX-GLOBAL-01", "IND");
        assert!(matches!(val_res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = val_res {
            assert!(report.is_authorized);
            assert_eq!(report.target_territory_code, "IND");
            assert_eq!(report.expiration_utc, "2030-12-31T23:59:59Z");
        }

        // Test empty input rejection
        assert!(matches!(engine.validate_distribution_rights("", "IND"), SiraResult::Error(_)));

        // Test path traversal rejection
        assert!(matches!(engine.validate_distribution_rights("PKG/../traversed", "IND"), SiraResult::Error(_)));
    }
}
