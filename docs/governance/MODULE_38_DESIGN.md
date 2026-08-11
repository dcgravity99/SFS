# MODULE 38 DESIGN SPECIFICATION: AUDIO STUDIO UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 38 establishes the **Audio Studio UI** (`apps/studio-ui/src/features/audio/`) for **Siragugal Film Studio**. It implements dialogue stem track management, waveform track visualization, Foley sound effect library browsers (`AssetId` handles), multi-track audio channel mixers (Volume dB sliders, Panning L/R, Mute, Solo), audio-to-scene timecode synchronization, and live IPC integration with `sira_engine_audio` (Module 23) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Audio Studio Workspace**: Master multi-track audio workstation layout coordinating Dialogue, Foley, Sound Effects (SFX), and Background Music (BGM) stems.
2. **Dialogue Stem Track Panel**: Character dialogue segment alignment panel showing `DialogueSegmentId`, character name, and start/end timecode offsets.
3. **Waveform Track Visualizer**: Dynamic canvas/SVG audio waveform amplitude display (`-48dB` to `+6dB`).
4. **Foley & Sound Effects Library Browser**: Searchable sound effect catalog panel binding `AssetId` audio handles stored in `asset_db`.
5. **Multi-Track Audio Mixer Panel**: Channel strip inspector supporting Gain sliders (`-inf` to `+6 dB`), Panning controls (`-100 Left` to `+100 Right`), Mute, and Solo toggles.
6. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all audio mixer controls.
7. **Sub-Engine IPC Integration**: Send audio track updates to `sira_engine_audio` via `StudioIpcService.executeEngineCommand('audio_update_track', ...)` and `audio_update_mixer`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 23 (`sira_engine_audio`), Module 19 (`sira_engine_actor`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 37 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_37_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 38 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/audio/types.ts)
export interface AudioTrackChannelView {
  track_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  track_type: 'Dialogue' | 'Foley' | 'SFX' | 'Music';
  volume_db: number; // -60.0 to +6.0
  pan: number; // -1.0 (Left) to +1.0 (Right)
  is_muted: boolean;
  is_solo: boolean;
  asset_id?: string;
}

export interface WaveformSegmentView {
  segment_id: string;
  start_ms: number;
  duration_ms: number;
  amplitude_peaks: number[]; // Normalized 0.0 - 1.0
}

// React Feature Components
export declare const AudioWorkspace: React.FC;
export declare const DialogueTrackPanel: React.FC;
export declare const WaveformViewer: React.FC;
export declare const SoundLibraryPanel: React.FC;
export declare const AudioMixerPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 38 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── audio.json
            │       └── en-US/
            │           └── audio.json
            └── features/
                └── audio/          # Audio Studio feature package
                    ├── types.ts    # Audio track & waveform UI models
                    ├── AudioWorkspace.tsx     # Master audio workspace container
                    ├── DialogueTrackPanel.tsx # Dialogue segment track manager
                    ├── WaveformViewer.tsx     # Waveform amplitude visualizer
                    ├── SoundLibraryPanel.tsx  # Foley & SFX asset picker
                    └── AudioMixerPanel.tsx    # Channel strip mixer (Gain, Pan, M/S)
```

---

## 6. Testing & Validation Strategy

1. **Audio Channel Strip Volume Slider Test**: Adjust Dialogue track volume slider to `-3dB`; verify `audio_update_track` IPC payload emits updated volume level cleanly.
2. **Waveform Peak Render Test**: Supply `amplitude_peaks` array; verify SVG peak bars render at correct heights.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`ஒலித் துறை`).

---

## 7. Acceptance Criteria

Module 38 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Audio Studio components render channel strips, waveform tracks, and sound effect libraries cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all audio workspace controls.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 38: Audio Studio UI**.
> 2. Upon your explicit approval, I will execute Module 38 implementation (`apps/studio-ui/src/features/audio/`).
