/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type AudioTrackType = 'Dialogue' | 'Foley' | 'SFX' | 'Music';

export interface AudioTrackChannelView {
  track_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  track_type: AudioTrackType;
  volume_db: number; // -60.0 to +6.0
  pan: number; // -1.0 (Left) to +1.0 (Right)
  is_muted: boolean;
  is_solo: boolean;
  asset_id?: string;
}

export interface WaveformSegmentView {
  segment_id: string;
  start_ms: number;
  duration_ms: number;
  amplitude_peaks: number[]; // Normalized 0.0 - 1.0
}

export interface SoundAssetReference {
  asset_id: string;
  display_name: LocalizedTextMap;
  category: 'Foley' | 'Ambience' | 'SFX' | 'Music';
}
