/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

export type WorkspaceMode =
  | 'story'
  | 'character'
  | 'actor'
  | 'scene'
  | 'director'
  | 'cinematography'
  | 'audio'
  | 'timeline'
  | 'prompts'
  | 'assets'
  | 'project'
  | 'render'
  | 'collaboration'
  | 'settings';


interface WorkspaceState {
  activeMode: WorkspaceMode;
  setMode: (mode: WorkspaceMode) => void;
}

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  activeMode: 'story',
  setMode: (mode) => set({ activeMode: mode }),
}));
