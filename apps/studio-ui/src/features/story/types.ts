/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export interface DialogueBlockView {
  character_name: string;
  parenthetical?: string;
  speech_text: string;
}

export interface ScriptSceneView {
  scene_number: number;
  heading: string;
  action_lines: string[];
  dialogue_blocks: DialogueBlockView[];
}

export interface StoryBeatView {
  beat_id: string;
  beat_type: string;
  scene_ids: number[];
  description: string;
}

export interface CharacterDialogueStat {
  character_name: string;
  line_count: number;
  word_count: number;
}
