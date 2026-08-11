/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type ShotType = 'Wide' | 'Medium' | 'CloseUp' | 'OverTheShoulder' | 'Establishing';

export interface ShotItemView {
  shot_id: string; // Machine-readable UUIDv7
  scene_id: number;
  shot_number: string;
  shot_type: ShotType;
  focal_length_mm: number;
  duration_frames: number;
  director_notes: LocalizedTextMap;
}

export interface DirectorIntentConfig {
  shot_id: string;
  intent_label: LocalizedTextMap;
  pacing_speed: 'Fast' | 'Moderate' | 'Slow' | 'Dramatic';
}

export interface CameraBlockingViewModel {
  camera_id: string;
  track_position: [number, number, number];
  target_character_id: string;
}
