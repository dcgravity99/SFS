/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

interface TimelineState {
  currentFrame: number;
  totalFrames: number;
  fps: number;
  isPlaying: boolean;
  setFrame: (frame: number) => void;
  togglePlay: () => void;
}

export const useTimelineStore = create<TimelineState>((set) => ({
  currentFrame: 0,
  totalFrames: 240,
  fps: 24,
  isPlaying: false,
  setFrame: (frame) => set({ currentFrame: frame }),
  togglePlay: () => set((state) => ({ isPlaying: !state.isPlaying })),
}));
