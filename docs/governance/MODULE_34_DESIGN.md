# MODULE 34 DESIGN SPECIFICATION: ACTOR STUDIO UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 34 establishes the **Actor Studio UI** (`apps/studio-ui/src/features/actor/`) for **Siragugal Film Studio**. It implements actor performance synthesis interfaces, voice model selection panels (`VoiceModelId`), viseme lip-sync timeline alignment visualizers (`sil`, `p`, `f`, `t`, `s`, `k`, `i`, `r`, `a`), emotional tone dictionary configurators, actor consistency distance meters, and live IPC integration with `sira_engine_actor` (Module 19) without adding unapproved rendering or AI video generation features.

---

## 2. Module Responsibilities & Core Features

1. **Actor Voice Model Selector**: Interactive panel for choosing synthesized voice profiles (`VoiceModelId`) bound to characters via `asset_db`.
2. **Viseme Lip-Sync Timeline Visualizer**: Timeline track visualizer displaying frame-accurate viseme phoneme timings aligned to dialogue segments.
3. **Emotional Tone & Dialect Configurator**: Emotional tone adjustment panel (`Pitch`, `Speed`, `Intensity`, `Emotion: Neutral / Dramatic / Angry / Melancholic`).
4. **Sub-Engine IPC Integration**: Send voice synthesis requests to `sira_engine_actor` via `StudioIpcService.executeEngineCommand('actor_synthesize_speech', ...)` and render viseme alignment tracks.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 19 (`sira_engine_actor`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 33 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_33_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 34 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/actor/types.ts)
export interface VisemeFrameView {
  frame_index: number;
  timecode_ms: number;
  viseme_code: 'sil' | 'p' | 'f' | 't' | 's' | 'k' | 'i' | 'r' | 'a';
  weight: number; // 0.0 to 1.0
}

export interface ActorPerformanceConfig {
  character_id: string;
  voice_model_id: string;
  emotional_tone: 'Neutral' | 'Dramatic' | 'Angry' | 'Melancholic';
  pitch_shift: number;
  speech_rate: number;
}

// React Feature Components
export declare const ActorVoiceSelector: React.FC;
export declare const VisemeTimelineVisualizer: React.FC;
export declare const EmotionalTonePanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 34 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            └── features/
                └── actor/          # Actor Studio feature package
                    ├── types.ts    # Actor UI & viseme data models
                    ├── ActorVoiceSelector.tsx       # Voice model selector
                    ├── VisemeTimelineVisualizer.tsx # Frame-accurate viseme timeline
                    └── EmotionalTonePanel.tsx       # Emotional tone & dialect panel
```

---

## 6. Testing & Validation Strategy

1. **Voice Selector Integration Test**: Select `voice-elevenlabs-v1`; verify `VoiceModelId` updates in actor performance state.
2. **Viseme Timeline Render Test**: Supply `VisemeFrameView` array; verify viseme code badges render at correct frame offsets.
3. **Emotional Tone Slider Test**: Adjust pitch shift slider; verify IPC payload contains updated numerical shift values.

---

## 7. Acceptance Criteria

Module 34 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Actor Studio components render voice profiles and viseme timelines cleanly.
3. IPC commands communicate with `sira_engine_actor` with WCAG 2.2 AA accessibility support.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 34: Actor Studio UI**.
> 2. Upon your explicit approval, I will execute Module 34 implementation (`apps/studio-ui/src/features/actor/`).
