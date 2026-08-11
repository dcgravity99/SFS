# MODULE 36 DESIGN SPECIFICATION: DIRECTOR STUDIO UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 36 establishes the **Director Studio UI** (`apps/studio-ui/src/features/director/`) for **Siragugal Film Studio**. It implements shot planning workspaces, interactive shot list managers (`Wide Shot`, `Medium Shot`, `Close-Up`, `Over The Shoulder`), director notes and script annotations panels, camera blocking diagram visualizers, scene-to-shot relationship mapping, and live IPC integration with `sira_engine_director` (Module 21) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Shot Planning Workspace**: Interactive shot planning dashboard mapping screenplay beats to camera shot specifications (`ShotType`, `CameraAngle`, `FocalLength`).
2. **Interactive Shot List Manager**: Reorderable shot inventory display showing shot IDs, duration, lens profiles, and director intent flags.
3. **Director Notes & Script Annotations Panel**: Rich text annotation panel binding director notes (`DirectorIntent`) to specific scene beats and shot items.
4. **Camera Blocking & Pacing Visualizer**: Diagram visualizer displaying camera placement trajectories and scene pacing meters (`Pacing: Fast / Dramatic / Slow`).
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all director workspace UI controls.
6. **Sub-Engine IPC Integration**: Send shot creation requests to `sira_engine_director` via `StudioIpcService.executeEngineCommand('director_create_shot', ...)` and `director_update_blocking`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 20 (`sira_engine_scene`), Module 21 (`sira_engine_director`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 35 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_35_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 36 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/director/types.ts)
export interface ShotItemView {
  shot_id: string; // Machine-readable UUIDv7
  scene_id: number;
  shot_number: string;
  shot_type: 'Wide' | 'Medium' | 'CloseUp' | 'OverTheShoulder' | 'Establishing';
  focal_length_mm: number;
  duration_frames: number;
  director_notes: LocalizedTextMap;
}

export interface DirectorIntentConfig {
  shot_id: string;
  intent_label: LocalizedTextMap;
  pacing_speed: 'Fast' | 'Moderate' | 'Slow' | 'Dramatic';
}

// React Feature Components
export declare const DirectorWorkspace: React.FC;
export declare const ShotListPanel: React.FC;
export declare const DirectorNotesPanel: React.FC;
export declare const CameraBlockingView: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 36 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── director.json
            │       └── en-US/
            │           └── director.json
            └── features/
                └── director/       # Director Studio feature package
                    ├── types.ts    # Director & shot UI models
                    ├── DirectorWorkspace.tsx   # Shot planning workspace
                    ├── ShotListPanel.tsx       # Interactive shot list manager
                    ├── DirectorNotesPanel.tsx  # Script annotations panel
                    └── CameraBlockingView.tsx  # Camera blocking visualizer
```

---

## 6. Testing & Validation Strategy

1. **Shot List Render Test**: Supply `ShotItemView` array; verify shot cards render with correct shot type badges and localized notes.
2. **Director Notes Annotation Test**: Add director note text; verify IPC payload emits structured `director_create_shot` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`இயக்குனர் திட்டம்`).

---

## 7. Acceptance Criteria

Module 36 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Director Studio components render shot lists and blocking diagrams cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all director workspace panels.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 36: Director Studio UI**.
> 2. Upon your explicit approval, I will execute Module 36 implementation (`apps/studio-ui/src/features/director/`).
