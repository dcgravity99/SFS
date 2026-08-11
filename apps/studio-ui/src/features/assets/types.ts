/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export type AssetCategory = 'Video' | 'Audio' | 'Model' | 'Texture' | 'Script';

export interface AssetMetadataView {
  asset_id: string; // Machine-readable UUIDv7 handle
  display_name: LocalizedTextMap;
  asset_category: AssetCategory;
  mime_type: string;
  file_size_bytes: number;
  sha256_checksum: string;
  created_at: string;
}

export interface StorageQuotaView {
  total_quota_bytes: number;
  used_bytes: number;
  cached_models_bytes: number;
}
