/*
============================================================================
Siragugal Film Studio
Copyright (C) 2026 Siragugal Film Studio Contributors
Licensed under Apache-2.0 or MIT.
============================================================================
*/

use regex::Regex;

pub struct RedactionEngine;

impl RedactionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn redact(&self, input: &str) -> String {
        let key_regex = Regex::new(r"sk-[a-zA-Z0-9]{20,}").unwrap();

        let sanitized = key_regex.replace_all(input, "[REDACTED_API_KEY]");

        let token_regex = Regex::new(r"(?i)bearer\s+[a-zA-Z0-9._\-]+").unwrap();

        let result = token_regex.replace_all(&sanitized, "Bearer [REDACTED_TOKEN]");

        result.to_string()
    }

    pub fn sanitize(input: &str) -> String {
        Self::new().redact(input)
    }
}
