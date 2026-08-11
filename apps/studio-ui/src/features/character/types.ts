/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface CharacterProfileView {
  character_id: string;
  name: string;
  role: string;
  voice_model_id?: string;
  lora_asset_id?: string; // AssetId reference only (zero absolute paths)
  visual_anchor_count: number;
  consistency_score: number; // 0.0 to 1.0
}

export interface CharacterCreatePayload {
  name: string;
  role: string;
}

export interface LoraBindPayload {
  character_id: string;
  lora_asset_id: string;
}
