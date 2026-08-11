/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

interface PreferencesState {
  theme: 'dark' | 'light' | 'high-contrast';
  reducedMotion: boolean;
  highContrastMode: boolean;
  setTheme: (theme: 'dark' | 'light' | 'high-contrast') => void;
}

export const usePreferencesStore = create<PreferencesState>((set) => ({
  theme: 'dark',
  reducedMotion: false,
  highContrastMode: false,
  setTheme: (theme) => set({ theme }),
}));
