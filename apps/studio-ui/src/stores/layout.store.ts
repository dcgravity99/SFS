/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

import { create } from 'zustand';

interface LayoutState {
  sidebarOpen: boolean;
  inspectorOpen: boolean;
  bottomPanelOpen: boolean;
  toggleSidebar: () => void;
  toggleInspector: () => void;
}

export const useLayoutStore = create<LayoutState>((set) => ({
  sidebarOpen: true,
  inspectorOpen: true,
  bottomPanelOpen: true,
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  toggleInspector: () => set((state) => ({ inspectorOpen: !state.inspectorOpen })),
}));
