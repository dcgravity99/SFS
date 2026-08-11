/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export interface StudioPreferencesView {
  primary_locale: 'ta-IN' | 'en-US';
  theme_mode: 'Dark' | 'Light' | 'System';
  auto_save_interval_mins: number;
  undo_history_depth: number;
  gpu_acceleration_enabled: boolean;
  vram_limit_mb: number;
}

export interface HardwareAccelerationConfigView {
  gpu_name: string;
  backend_api: 'CUDA' | 'Vulkan' | 'DirectML';
  is_enabled: boolean;
  max_vram_allocation_mb: number;
}

export interface SecurityAuditEventView {
  audit_id: string; // Machine-readable UUIDv7
  standard_name: string; // e.g. "OWASP ASVS L2"
  status: 'Compliant' | 'Warning' | 'NonCompliant';
  verified_at: string;
}
