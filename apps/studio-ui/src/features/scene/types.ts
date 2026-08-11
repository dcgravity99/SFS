/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string; // Tamil (Primary)
  "en-US": string; // English (Secondary Fallback)
}

export interface SceneTransformView {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface SceneNodeView {
  node_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  node_type: 'Camera' | 'Character' | 'Prop' | 'Marker' | 'Environment';
  transform: SceneTransformView;
  asset_id?: string;
}

export interface PropAssetReference {
  asset_id: string;
  display_name: LocalizedTextMap;
  category: string;
}

export interface CameraFrustumValidationView {
  is_occluded: boolean;
  occluding_node_ids: string[];
}
