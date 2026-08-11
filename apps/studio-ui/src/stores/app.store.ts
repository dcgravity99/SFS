/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

interface AppState {
  isInitialized: boolean;
  isGpuAccelerated: boolean;
  developerMode: boolean;
  activeJobCount: number;
  setInitialized: (initialized: boolean) => void;
}

export const useAppStore = create<AppState>((set) => ({
  isInitialized: false,
  isGpuAccelerated: true,
  developerMode: false,
  activeJobCount: 0,
  setInitialized: (initialized) => set({ isInitialized: initialized }),
}));
