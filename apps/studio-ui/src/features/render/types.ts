/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type RenderStatus = 'Queued' | 'Rendering' | 'Completed' | 'Failed';
export type RenderPriority = 'High' | 'Normal' | 'Background';

export interface RenderJobView {
  job_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  priority: RenderPriority;
  status: RenderStatus;
  current_frame: number;
  total_frames: number;
  output_format: string; // e.g. "EXR Sequence 16-bit", "ProRes 4444"
  eta_seconds: number;
}

export interface GpuResourceTelemetryView {
  gpu_name: string;
  vram_used_bytes: number;
  vram_total_bytes: number;
  gpu_utilization_percent: number;
}

export interface RenderProgressView {
  job_id: string;
  current_frame: number;
  total_frames: number;
  current_pass: string;
  eta_seconds: number;
}

export interface RenderRecoveryView {
  job_id: string;
  last_checkpoint_frame: number;
  error_message: LocalizedTextMap;
}
