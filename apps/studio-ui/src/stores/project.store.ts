/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

interface ProjectState {
  projectId: string | null;
  projectName: string;
  isDirty: boolean;
  setProject: (id: string, name: string) => void;
}

export const useProjectStore = create<ProjectState>((set) => ({
  projectId: 'proj-demo-1001',
  projectName: 'Untitled Siragugal Film Project',
  isDirty: false,
  setProject: (id, name) => set({ projectId: id, projectName: name, isDirty: false }),
}));
