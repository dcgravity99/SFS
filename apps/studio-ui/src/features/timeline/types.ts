/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type TrackType = 'Video' | 'Audio' | 'Subtitle';

export interface TimelineClipView {
  clip_id: string; // Machine-readable UUIDv7
  track_id: string;
  display_name: LocalizedTextMap;
  start_frame: number;
  duration_frames: number;
  in_point_frame: number;
  out_point_frame: number;
  asset_id?: string;
}

export interface TimelineTrackHeaderView {
  track_id: string;
  track_name: LocalizedTextMap;
  track_type: TrackType;
  is_locked: boolean;
  is_visible: boolean;
}
