/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export interface FilmProjectMetadataView {
  project_id: string; // Machine-readable UUIDv7
  title: LocalizedTextMap;
  synopsis: LocalizedTextMap;
  director_name: string;
  production_house: string;
  target_aspect_ratio: string; // e.g. "2.39:1"
  target_fps: number;
  created_at: string;
}

export interface ProductionMilestoneView {
  milestone_id: string;
  name: LocalizedTextMap;
  progress_percent: number; // 0 to 100
  is_completed: boolean;
}

export interface ProjectCheckpointView {
  checkpoint_id: string;
  version_tag: string;
  description: LocalizedTextMap;
  timestamp: string;
}
