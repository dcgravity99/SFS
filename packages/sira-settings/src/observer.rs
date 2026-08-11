/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingChangeEvent {
    pub transaction_id: String,
    pub timestamp: String,
    pub key: String,
    pub old_value: String,
    pub new_value: String,
    pub requires_restart: bool,
}

pub type SettingChangeCallback = Box<dyn Fn(&SettingChangeEvent) + Send + Sync>;

pub struct SettingsObserverBus {
    listeners: RwLock<Vec<SettingChangeCallback>>,
}

impl SettingsObserverBus {
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(Vec::new()),
        }
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&SettingChangeEvent) + Send + Sync + 'static,
    {
        if let Ok(mut list) = self.listeners.write() {
            list.push(Box::new(callback));
        }
    }

    pub fn notify(&self, event: &SettingChangeEvent) {
        if let Ok(list) = self.listeners.read() {
            for callback in list.iter() {
                callback(event);
            }
        }
    }
}
