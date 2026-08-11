# MODULE 43 DESIGN SPECIFICATION: RENDER QUEUE & PRODUCTION CONTROL UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 43 establishes the **Render Queue & Production Control UI** (`apps/studio-ui/src/features/render/`) for **Siragugal Film Studio**. It implements production batch render queue managers (`RenderJobId`), active render job progress monitors (Estimated Time Remaining, Frame Completion, Render Target Resolution 4K/8K), GPU & VRAM resource utilization gauges, failed job recovery panels, and live IPC integration with `sira_render_engine` (Module 06) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Render Queue Workspace**: Master production render control workspace coordinating queued, active, and completed render jobs.
2. **Render Queue Panel**: Interactive job queue list (`RenderJobCard.tsx`) displaying job priority (`High`, `Normal`, `Background`), status (`Queued`, `Rendering`, `Completed`, `Failed`), and target output format (`ProRes 4444`, `EXR Sequence`, `MP4 H.265`).
3. **Render Progress Monitor**: High-precision progress gauge tracking frame-by-frame rendering (`Frame 142/360`), current pass, and ETA countdown.
4. **GPU & Resource Monitoring Panel**: Hardware telemetry gauge displaying GPU VRAM allocation, GPU compute utilization %, and memory bandwidth.
5. **Render Failure Recovery Panel**: Crash-resilient recovery panel allowing failed render job retries from last saved checkpoint frame without re-rendering completed frames.
6. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all render queue controls.
7. **Sub-Engine IPC Integration**: Send render jobs to `sira_render_engine` via `StudioIpcService.executeEngineCommand('render_submit_job', ...)` and `render_cancel_job`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 06 (`sira_render_engine`), Module 08 (`sira_core`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 42 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_42_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 43 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/render/types.ts)
export interface RenderJobView {
  job_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  priority: 'High' | 'Normal' | 'Background';
  status: 'Queued' | 'Rendering' | 'Completed' | 'Failed';
  current_frame: number;
  total_frames: number;
  output_format: string; // e.g. "EXR Sequence 16-bit"
  eta_seconds: number;
}

export interface GpuResourceTelemetryView {
  gpu_name: string;
  vram_used_bytes: number;
  vram_total_bytes: number;
  gpu_utilization_percent: number;
}

// React Feature Components
export declare const RenderWorkspace: React.FC;
export declare const RenderQueuePanel: React.FC;
export declare const RenderJobCard: React.FC;
export declare const RenderProgressMonitor: React.FC;
export declare const RenderResourcePanel: React.FC;
export declare const RenderRecoveryPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 43 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── render.json
            │       └── en-US/
            │           └── render.json
            └── features/
                └── render/         # Render Queue feature package
                    ├── types.ts    # Render job & telemetry UI models
                    ├── RenderWorkspace.tsx       # Master render workspace
                    ├── RenderQueuePanel.tsx     # Batch render queue list
                    ├── RenderJobCard.tsx        # Render job item card
                    ├── RenderProgressMonitor.tsx # Frame progress & ETA monitor
                    ├── RenderResourcePanel.tsx   # GPU & VRAM telemetry gauge
                    └── RenderRecoveryPanel.tsx   # Failed job recovery panel
```

---

## 6. Testing & Validation Strategy

1. **Render Job Submission IPC Test**: Click Submit Render Job; verify IPC payload emits `render_submit_job` command cleanly.
2. **Frame Progress Render Test**: Update frame progress to `142/360`; verify progress bar and ETA calculate cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`வெளியீட்டுத் துறை (Render Studio)`).

---

## 7. Acceptance Criteria

Module 43 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Render Queue components render batch job lists, progress monitors, and GPU telemetry panels cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all render queue controls.
4. Zero unapproved local video rendering code is present in React UI.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 43: Render Queue & Production Control UI**.
> 2. Upon your explicit approval, I will execute Module 43 implementation (`apps/studio-ui/src/features/render/`).
