/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export enum SiraErrorCode {
  // System Core Range (1000 - 1999)
  UNKNOWN_SYSTEM_ERROR = 1000,
  WORKSPACE_INITIALIZATION_FAILED = 1001,
  CONFIG_SCHEMA_INVALID = 1002,

  // Hardware Abstraction Layer Range (2000 - 2999)
  HAL_DEVICE_NOT_FOUND = 2000,
  CUDA_VRAM_ALLOCATION_OOM = 2015,

  // SIRA AI Core & Model Registry Range (3000 - 3999)
  MODEL_NOT_FOUND = 3000,
  MODEL_CHECKSUM_VERIFICATION_FAILED = 3008,

  // Project Engine Range (4000 - 4999)
  SFSP_MANIFEST_CORRUPTED = 4002,

  // Workflow Graph Engine Range (5000 - 5999)
  WORKFLOW_DAG_CYCLE_DETECTED = 5012,

  // Plugin Runtime Range (6000 - 6999)
  PLUGIN_PERMISSION_DENIED = 6004,

  // Render Scheduler Range (7000 - 7999)
  RENDER_CHECKPOINT_RESUME_FAILED = 7009,
}

export interface SiraError {
  code: SiraErrorCode;
  errorName: string;
  category: string;
  severity: 'FATAL' | 'ERROR' | 'WARNING';
  isRecoverable: boolean;
  correlationId?: string;
  jobId?: string;
  i18nKey: string;
  context?: Record<string, unknown>;
  suggestedActionKey?: string;
}
