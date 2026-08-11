# MODULE 31 DESIGN SPECIFICATION: STUDIO UI FRAMEWORK & DESKTOP SHELL INTEGRATION
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 31 initiates **Phase 3 (Application & Product Development)** for **Siragugal Film Studio**. It establishes the core frontend presentation architecture (`apps/studio-ui/`), Tauri 2.x IPC command router bridge (`sira_studio_app`), React 19 state architecture, glassmorphic dark theme design system (Tailwind CSS, WCAG 2.2 AA accessibility, custom CSS design tokens), and multi-window workspace layout manager without adding unapproved feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Tauri 2.x Desktop IPC Bridge**: Bind React 19 frontend components to Rust backend engines (Modules 00–30) via strongly typed IPC commands (`invoke`) and asynchronous event listeners (`listen`).
2. **React 19 State & Context Architecture**: Establish global studio state stores (Project state, Timeline state, Sub-Engine state, Experience Notification state) with zero unnecessary re-renders.
3. **Design System & Theme Tokens**: Implement a dark mode visual theme featuring glassmorphism, micro-animations, custom typography (Inter, Outfit), dynamic layout grids, and WCAG 2.2 AA contrast compliance.
4. **Multi-Panel Workspace Layout Coordinator**: Provide drag-and-droppable multi-dock panel views (Screenplay Editor panel, 3D Scene Viewport, NLE Timeline, Node Graph, Render Monitor).
5. **Security & Content Security Policy (CSP)**: Enforce strict CSP headers, IPC input sanitization, and XSS protection.

---

## 3. Module Dependencies

- **Backend Dependencies**: All Phase 1 & Phase 2 Modules 00 - 30 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `plugin_runtime`, `resource_manager`, `cache_manager`, `sira_engine_*`, `sira_studio_app`).
- **Frontend Tech Stack**: Tauri 2.x, React 19, TypeScript Strict Mode, Tailwind CSS, Lucide Icons, Vite 5.

---

## 4. Public APIs & IPC Contracts

Module 31 defines strongly typed TypeScript and Tauri 2.x IPC contracts:

```typescript
// TypeScript Contract (src/types/ipc.ts)
export interface StudioBootstrapConfig {
  project_file_path?: string;
  enable_gpu_acceleration: boolean;
  developer_mode: boolean;
}

export interface IpcCommandRequest<T = unknown> {
  command_name: string;
  payload: T;
  correlation_id: string;
}

export interface IpcCommandResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
  };
}

// Frontend Services API
export class StudioIpcService {
  static async bootstrapStudio(config: StudioBootstrapConfig): Promise<IpcCommandResponse<void>>;
  static async executeEngineCommand<TIn, TOut>(command: string, payload: TIn): Promise<IpcCommandResponse<TOut>>;
  static subscribeToExperienceEvents(callback: (event: unknown) => void): () => void;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design specification, Module 31 will create the following frontend application structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/                  # React 19 & Tauri 2.x presentation app
        ├── package.json
        ├── tsconfig.json
        ├── vite.config.ts
        ├── tailwind.config.js
        ├── index.html
        └── src/
            ├── main.tsx            # Application entry point
            ├── App.tsx             # Root layout container
            ├── index.css           # Design tokens & glassmorphic styles
            ├── services/           # Tauri 2.x IPC bridge & event bus listeners
            │   └── ipc.service.ts
            ├── stores/             # React 19 / Zustand state stores
            │   ├── project.store.ts
            │   └── workspace.store.ts
            ├── components/         # Core UI design components & layout panels
            │   ├── common/
            │   │   ├── Button.tsx
            │   │   ├── Card.tsx
            │   │   └── Modal.tsx
            │   └── layout/
            │       ├── Header.tsx
            │       ├── Sidebar.tsx
            │       ├── WorkspacePanel.tsx
            │       └── StatusBar.tsx
            └── styles/
                └── theme.css
```

---

## 6. Security Implications & Compliance

- **OWASP ASVS Level 2**: Enforce strict Content Security Policy (`default-src 'self'`), disable `eval()`, sanitize all dynamic text inputs, and restrict IPC commands to whitelisted Rust functions.
- **IPC Input Validation**: Validate all frontend payloads using Zod / TypeScript schemas before emitting across the Tauri IPC bridge.
- **Zero Secrets in Frontend**: Zero API keys or bearer tokens stored in web local storage or JS memory.

---

## 7. Testing Strategy

1. **React Component Unit Tests**: Test layout components and UI controls using React Testing Library.
2. **IPC Service Mock Integration Tests**: Mock Tauri `invoke` calls; verify state stores update correctly on successful responses and handle error codes cleanly.
3. **Accessibility WCAG 2.2 AA Audit**: Verify ARIA roles, keyboard focus traps, and color contrast ratios (`>= 4.5:1`).

---

## 8. Performance Budgets

- **Initial UI Render Latency**: `< 50 ms`
- **IPC Round-Trip Overhead**: `< 1.0 ms`
- **Frame Rate during Animations**: Continuous 60 FPS
- **Memory Footprint**: Bounded `< 150 MB` for frontend renderer process

---

## 9. Acceptance Criteria

Module 31 is accepted when:
1. `apps/studio-ui/` builds cleanly with zero TypeScript or build errors.
2. Tauri 2.x IPC bridge communicates bidirectionally with `sira_studio_app`.
3. Dark mode glassmorphic UI framework renders at 60 FPS with full keyboard navigation and ARIA accessibility compliance.
4. Zero unapproved application features are present.

---

## 10. Next Action

> [!IMPORTANT]
> Per the mandatory Phase 3 workflow rule:
> 1. Please review this design specification for **Module 31: Studio UI Framework & Desktop Shell Integration**.
> 2. Upon your explicit approval, I will proceed with Module 31 implementation (`apps/studio-ui`).
