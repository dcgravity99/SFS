/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use std::sync::RwLock;

pub type ConfigChangeCallback = Box<dyn Fn(&str, &str) + Send + Sync>;

pub struct ConfigObserverBus {
    listeners: RwLock<Vec<ConfigChangeCallback>>,
}

impl ConfigObserverBus {
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(Vec::new()),
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&str, &str) + Send + Sync + 'static,
    {
        if let Ok(mut list) = self.listeners.write() {
            list.push(Box::new(callback));
        }
    }

    pub fn notify(&self, key: &str, new_value: &str) {
        if let Ok(list) = self.listeners.read() {
            for callback in list.iter() {
                callback(key, new_value);
            }
        }
    }
}
