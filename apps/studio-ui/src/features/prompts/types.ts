/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface LocalizedTextMap {
  "ta-IN": string;
  "en-US": string;
}

export interface AiPromptSpecView {
  prompt_id: string; // Machine-readable UUIDv7
  positive_prompt: string;
  negative_prompt: string;
  model_id: string; // ModelId handle
  lora_asset_ids: string[]; // AssetId handles
  cfg_scale: number; // 1.0 to 20.0
  steps: number; // 10 to 150
  seed: number;
  sampler_name: string;
}

export interface PromptTemplatePresetView {
  preset_id: string;
  name: LocalizedTextMap;
  description: LocalizedTextMap;
  positive_template: string;
}
