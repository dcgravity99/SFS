/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRole {
  Administrator,
  Director,
  Producer,
  Cinematographer,
  Animator,
  AudioEngineer,
  Editor,
  Viewer,
}

pub fn validate_access_permission(artist_role: &str, resource_action: &str) -> Result<bool, String> {
  match artist_role {
    "Administrator" | "Director" | "Producer" => Ok(true),
    "Viewer" => {
      if resource_action.starts_with("write") || resource_action.starts_with("delete") {
        Ok(false)
      } else {
        Ok(true)
      }
    }
    _ => Ok(true),
  }
}
