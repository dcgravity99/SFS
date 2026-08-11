/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface IpcRequestEnvelope<T = unknown> {
  request_id: string;      // UUIDv7
  correlation_id: string;  // UUIDv7
  schema_version: string;  // "1.0.0"
  timestamp_ms: number;
  command: string;         // Whitelisted command identifier
  payload: T;
}

export interface StandardizedIpcError {
  code: string;
  error_name: string;
  category: string;
  message: string;
  is_recoverable: boolean;
}

export interface IpcResponseEnvelope<T = unknown> {
  request_id: string;
  correlation_id: string;
  schema_version: string;
  timestamp_ms: number;
  success: boolean;
  data?: T;
  error?: StandardizedIpcError;
}

export interface ExperienceEventV1 {
  event_id: string;
  event_version: string;
  timestamp_ms: number;
  correlation_id: string;
  source_module: string;
  severity: "Info" | "Success" | "Warning" | "Error" | "Critical";
  event_category: string;
  payload_json: string;
}

export interface StudioBootstrapConfig {
  project_file_path?: string;
  enable_gpu_acceleration: boolean;
  developer_mode: boolean;
}
