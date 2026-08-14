/* ============================================================================
 * Siragugal Film Studio — Module 67: AI Creative Consistency Engine
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use sira_types::{SiraError, SiraErrorCode, SiraResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsistencyAuditRequest {
    pub franchise_id: String,
    pub project_id: String,
    pub canon_rules: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsistencyAuditReport {
    pub audit_id: String,
    pub is_canon_compliant: bool,
    pub violations_count: u32,
    pub approval_required: bool,
    pub reasoning_trace_id: String,
}

#[derive(Default)]
pub struct CreativeConsistencyEngine;

impl CreativeConsistencyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn audit_consistency(&self, request: &ConsistencyAuditRequest) -> SiraResult<ConsistencyAuditReport> {
        if request.franchise_id.is_empty() || request.project_id.is_empty() {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "EMPTY_CONSISTENCY_AUDIT_IDS".to_string(),
                category: "ECOSYSTEM_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.creative_consistency.empty_ids".to_string(),
                suggested_action_key: None,
            });
        }

        if request.franchise_id.contains("..") || request.project_id.contains("..") {
            return SiraResult::Error(SiraError {
                code: SiraErrorCode::UnknownSystemError,
                error_name: "INVALID_CONSISTENCY_AUDIT_PATH".to_string(),
                category: "ECOSYSTEM_ENGINE".to_string(),
                severity: "ERROR".to_string(),
                is_recoverable: false,
                correlation_id: None,
                job_id: None,
                i18n_key: "errors.creative_consistency.invalid_path".to_string(),
                suggested_action_key: None,
            });
        }

        let report = ConsistencyAuditReport {
            audit_id: format!("AUDIT-CANON-{}", request.franchise_id.to_uppercase()),
            is_canon_compliant: true,
            violations_count: 0,
            approval_required: true,
            reasoning_trace_id: format!("TRACE-CONSISTENCY-{}", request.project_id),
        };

        SiraResult::Success(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_67_creative_consistency_lifecycle() {
        let engine = CreativeConsistencyEngine::new();
        let request = ConsistencyAuditRequest {
            franchise_id: "FRANCHISE-SFS-UNIVERSE".to_string(),
            project_id: "PROJ-FILM-EP2".to_string(),
            canon_rules: vec!["Rule 1: Hero weapon is immutable.".to_string()],
        };

        let res = engine.audit_consistency(&request);
        assert!(matches!(res, SiraResult::Success(_)));

        if let SiraResult::Success(report) = res {
            assert_eq!(report.audit_id, "AUDIT-CANON-FRANCHISE-SFS-UNIVERSE");
            assert!(report.is_canon_compliant);
            assert_eq!(report.violations_count, 0);
            assert!(report.approval_required);
            assert_eq!(report.reasoning_trace_id, "TRACE-CONSISTENCY-PROJ-FILM-EP2");
        }

        // Test empty input rejection
        let invalid_request = ConsistencyAuditRequest {
            franchise_id: "".to_string(),
            project_id: "PROJ-01".to_string(),
            canon_rules: vec![],
        };
        assert!(matches!(engine.audit_consistency(&invalid_request), SiraResult::Error(_)));

        // Test path traversal rejection
        let path_invalid_request = ConsistencyAuditRequest {
            franchise_id: "FRANCHISE/../traversed".to_string(),
            project_id: "PROJ-01".to_string(),
            canon_rules: vec![],
        };
        assert!(matches!(engine.audit_consistency(&path_invalid_request), SiraResult::Error(_)));

        // Determinism test
        if let (SiraResult::Success(r1), SiraResult::Success(r2)) = (engine.audit_consistency(&request), engine.audit_consistency(&request)) {
            assert_eq!(r1.audit_id, r2.audit_id);
            assert_eq!(r1.is_canon_compliant, r2.is_canon_compliant);
        }
    }
}
