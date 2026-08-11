# MODULE 42 DESIGN SPECIFICATION: PROJECT MANAGEMENT & WORKSPACE ORCHESTRATION UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 42 establishes the **Project Management & Workspace Orchestration UI** (`apps/studio-ui/src/features/project/`) for **Siragugal Film Studio**. It implements overall film project dashboards, film metadata inspectors (Title, Synopsis, Director, Production House, Target Release Date), milestone production progress trackers, version history & backup checkpoint panels, and live IPC integration with `sira_core` (Module 08) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Project Management Workspace**: Master project orchestration container tracking project lifecycle, scene count, and production status.
2. **Film Project Dashboard**: Overview dashboard displaying active scene statistics, total shot count, render progress, and asset count.
3. **Film Metadata Inspector Panel**: Detailed form panel editing film title (`ta-IN` / `en-US`), director attribution, aspect ratio (`2.39:1 Anamorphic`), and target frame rate.
4. **Production Milestone Tracker**: Visual milestone progress bar tracking Pre-Production, Scripting, Scene Assembly, Cinematography, Audio Mixing, and Final Master Render.
5. **Version History & Checkpoint Viewer**: List panel viewing project snapshot checkpoints (`v1.0.0-alpha`, `v1.1.0-rc1`) and backup timestamps.
6. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all project management controls.
7. **Sub-Engine IPC Integration**: Send project metadata updates to `sira_core` via `StudioIpcService.executeEngineCommand('project_update_metadata', ...)` and `project_create_checkpoint`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 08 (`sira_core`), Module 05 (`sira_asset_db`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 41 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_41_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 42 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/project/types.ts)
export interface FilmProjectMetadataView {
  project_id: string; // Machine-readable UUIDv7
  title: LocalizedTextMap;
  synopsis: LocalizedTextMap;
  director_name: string;
  production_house: string;
  target_aspect_ratio: string; // e.g. "2.39:1"
  target_fps: number;
  created_at: string;
}

export interface ProductionMilestoneView {
  milestone_id: string;
  name: LocalizedTextMap;
  progress_percent: number; // 0 to 100
  is_completed: boolean;
}

export interface ProjectCheckpointView {
  checkpoint_id: string;
  version_tag: string;
  description: LocalizedTextMap;
  timestamp: string;
}

// React Feature Components
export declare const ProjectWorkspace: React.FC;
export declare const ProjectDashboard: React.FC;
export declare const ProjectMetadataPanel: React.FC;
export declare const ProductionTracker: React.FC;
export declare const VersionHistoryPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 42 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── project.json
            │       └── en-US/
            │           └── project.json
            └── features/
                └── project/        # Project Management feature package
                    ├── types.ts    # Project metadata & checkpoint UI models
                    ├── ProjectWorkspace.tsx     # Master project workspace
                    ├── ProjectDashboard.tsx     # Film project overview dashboard
                    ├── ProjectMetadataPanel.tsx # Title, director & specs panel
                    ├── ProductionTracker.tsx    # Production milestone progress bar
                    └── VersionHistoryPanel.tsx  # Checkpoint & backup history
```

---

## 6. Testing & Validation Strategy

1. **Project Dashboard Render Test**: Render dashboard; verify scene count and shot statistics calculate cleanly.
2. **Project Metadata Update IPC Test**: Modify film title; verify IPC payload emits `project_update_metadata` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`திரைப்படத் திட்டம் (Project Workspace)`).

---

## 7. Acceptance Criteria

Module 42 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Project Management components render dashboards, metadata panels, and milestone trackers cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all project management controls.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 42: Project Management & Workspace Orchestration UI**.
> 2. Upon your explicit approval, I will execute Module 42 implementation (`apps/studio-ui/src/features/project/`).
