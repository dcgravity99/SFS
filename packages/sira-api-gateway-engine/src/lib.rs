/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

pub mod gateway_router;
pub mod service_registry;
pub mod mtls_manager;
pub mod rate_limiter;
pub mod contract_validator;
pub mod request_auditor;

pub use gateway_router::route_secure_request;
pub use service_registry::register_service;
pub use mtls_manager::authenticate_mtls_service;
pub use rate_limiter::check_rate_limit;
pub use contract_validator::validate_api_contract;
pub use request_auditor::log_gateway_request;
