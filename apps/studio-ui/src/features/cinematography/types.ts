/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export interface LensProfileView {
  lens_profile_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  focal_length_mm: number;
  max_aperture: number;
  squeeze_factor: number; // 1.0 (Spherical), 2.0 (Anamorphic)
}

export interface CameraSettingsView {
  camera_id: string;
  lens_profile_id: string;
  focal_length_mm: number;
  aperture_fstop: number; // 1.4, 2.8, 5.6
  focus_distance_m: number;
  shutter_angle_deg: number;
  iso_rating: number;
}

export interface LightingProfileView {
  key_light_intensity: number; // 0.0 to 1.0
  fill_light_intensity: number;
  back_light_intensity: number;
  color_temperature_kelvin: number; // 2700K to 6500K
}

export interface DepthOfFieldView {
  near_focus_limit_m: number;
  far_focus_limit_m: number;
  hyperfocal_distance_m: number;
  bokeh_blur_factor: number;
}
