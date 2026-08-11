/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::collections::HashMap;

pub struct PhoneticDictionary {
    pronunciations: HashMap<String, String>,
}

impl PhoneticDictionary {
    pub fn new() -> Self {
        Self {
            pronunciations: HashMap::new(),
        }
    }

    pub fn register(&mut self, word: &str, phonemes: &str) {
        self.pronunciations.insert(word.to_lowercase(), phonemes.to_string());
    }
}
