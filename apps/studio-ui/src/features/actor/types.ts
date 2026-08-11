/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

export type VisemeCode = 'sil' | 'p' | 'f' | 't' | 's' | 'k' | 'i' | 'r' | 'a';

export interface VisemeFrameView {
  frame_index: number;
  timecode_ms: number;
  viseme_code: VisemeCode;
  weight: number; // 0.0 to 1.0
}

export interface ActorPerformanceConfig {
  character_id: string;
  voice_model_id: string;
  emotional_tone: 'Neutral' | 'Dramatic' | 'Angry' | 'Melancholic';
  pitch_shift: number; // -12 to +12 semitones
  speech_rate: number; // 0.5 to 2.0x
}

export interface ActorSynthesizePayload {
  character_id: string;
  voice_model_id: string;
  dialogue_text: string;
  emotional_tone: string;
}
