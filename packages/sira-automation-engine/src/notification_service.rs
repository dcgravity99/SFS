/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalNotification {
  pub notification_id: String,
  pub title: String,
  pub message: String,
  pub is_dispatched: bool,
}

pub fn send_local_notification(title: &str, message: &str) -> LocalNotification {
  LocalNotification {
    notification_id: "notif-local-057".to_string(),
    title: title.to_string(),
    message: message.to_string(),
    is_dispatched: true,
  }
}
