/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { SiraError } from './errors';

export type AsyncJobStatus = 'PENDING' | 'RUNNING' | 'PAUSED' | 'CANCELLED' | 'COMPLETED' | 'FAILED';

export type SiraResult<T> =
  | { status: 'SUCCESS'; data: T }
  | { status: 'PARTIAL_SUCCESS'; data: T; warnings: SiraError[] }
  | { status: 'ERROR'; error: SiraError }
  | { status: 'PROGRESS'; progress: number; stage: string }
  | { status: 'CANCELLED'; reason: string };
