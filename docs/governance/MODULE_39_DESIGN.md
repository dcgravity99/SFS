# MODULE 39 DESIGN SPECIFICATION: NLE TIMELINE EDITOR UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 39 establishes the **NLE Timeline Editor UI** (`apps/studio-ui/src/features/timeline/`) for **Siragugal Film Studio**. It implements professional non-linear video editing (NLE) timeline interfaces, multi-track timeline canvases (Video V1-V4, Audio A1-A4, Subtitle T1 tracks), SMPTE timecode rulers (`00:00:00:00`), razor split & clip trimming tools, playhead scrubbing controls, track locking & visibility toggles, and live IPC integration with `sira_engine_timeline` (Module 24) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **NLE Timeline Master Workspace**: Master non-linear multi-track editing workspace coordinating video, audio, and subtitle timeline tracks.
2. **SMPTE Timecode Ruler & Playhead**: Frame-accurate SMPTE timecode ruler (`00:01:24:12` at 24 FPS) with interactive playhead scrubbing controls.
3. **Multi-Track Clip Editor & Razor Trimming Tools**: Clip dragging, razor split, in/out trim point adjustments (`TrimIn`, `TrimOut`), and ripple edit controls.
4. **Track Locking & Visibility Inspector**: Track header controls for locking tracks (`is_locked`), toggling visibility (`is_visible`), and soloing tracks.
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all timeline controls.
6. **Sub-Engine IPC Integration**: Send timeline edit actions to `sira_engine_timeline` via `StudioIpcService.executeEngineCommand('timeline_split_clip', ...)` and `timeline_update_clip_trim`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 24 (`sira_engine_timeline`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useTimelineStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 38 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_38_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 39 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/timeline/types.ts)
export interface TimelineClipView {
  clip_id: string; // Machine-readable UUIDv7
  track_id: string;
  display_name: LocalizedTextMap;
  start_frame: number;
  duration_frames: number;
  in_point_frame: number;
  out_point_frame: number;
  asset_id?: string;
}

export interface TimelineTrackHeaderView {
  track_id: string;
  track_name: LocalizedTextMap;
  track_type: 'Video' | 'Audio' | 'Subtitle';
  is_locked: boolean;
  is_visible: boolean;
}

// React Feature Components
export declare const TimelineWorkspace: React.FC;
export declare const TimecodeRuler: React.FC;
export declare const MultiTrackCanvas: React.FC;
export declare const TrimmingToolsPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 39 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── timeline.json
            │       └── en-US/
            │           └── timeline.json
            └── features/
                └── timeline/       # NLE Timeline feature package
                    ├── types.ts    # Timeline clip & track UI models
                    ├── TimelineWorkspace.tsx   # Master NLE timeline workspace
                    ├── TimecodeRuler.tsx       # SMPTE timecode ruler
                    ├── MultiTrackCanvas.tsx    # Multi-track video/audio canvas
                    └── TrimmingToolsPanel.tsx  # Razor split & trim controls
```

---

## 6. Testing & Validation Strategy

1. **Timecode Ruler Frame Calculation Test**: Scrub playhead to frame 48 at 24 FPS; verify SMPTE display reads `00:00:02:00`.
2. **Razor Split IPC Test**: Trigger razor split command at current playhead; verify IPC payload emits `timeline_split_clip` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`நேரக்கோடு (Timeline)`).

---

## 7. Acceptance Criteria

Module 39 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. NLE Timeline components render multi-track clips, timecode rulers, and razor tools cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all timeline controls.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 39: NLE Timeline Editor UI**.
> 2. Upon your explicit approval, I will execute Module 39 implementation (`apps/studio-ui/src/features/timeline/`).
