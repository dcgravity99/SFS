# MODULE 37 DESIGN SPECIFICATION: CINEMATOGRAPHY STUDIO UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 37 establishes the **Cinematography Studio UI** (`apps/studio-ui/src/features/cinematography/`) for **Siragugal Film Studio**. It implements camera optics controls, lens profile management (`Anamorphic 35mm`, `Spherical 50mm`, `Prime 85mm`), depth of field (DoF) aperture inspectors (`f/1.4`, `f/2.8`, `f/5.6`), three-point lighting intensity controls, color temperature sliders (`2700K` to `6500K`), and live IPC integration with `sira_engine_cinematography` (Module 22) following the Tamil-first (`ta-IN`) globalization architecture.

---

## 2. Module Responsibilities & Core Features

1. **Cinematography Optics Workspace**: Master workspace displaying camera optics parameters, focal length, aperture f-stops, shutter angle (`180°`), and ISO rating.
2. **Camera Lens Profile Selector**: Interactive panel for choosing optical lens profiles (`LensProfileId`) bound to project asset metadata via `asset_db`.
3. **Depth of Field (DoF) Visualizer**: Interactive gauge visualizer calculating near/far focus limits and bokeh blurring intensity.
4. **Three-Point Lighting & Color Temp Inspector**: Control sliders for Key Light, Fill Light, Backlight intensity, and Kelvin color temperature (`3200K Warm` / `5600K Daylight`).
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all optics and lighting controls.
6. **Sub-Engine IPC Integration**: Send camera parameter updates to `sira_engine_cinematography` via `StudioIpcService.executeEngineCommand('cinematography_update_camera', ...)` and `cinematography_update_lighting`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 22 (`sira_engine_cinematography`), Module 21 (`sira_engine_director`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 36 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_36_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 37 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/cinematography/types.ts)
export interface CameraOpticsConfigView {
  camera_id: string;
  lens_profile_id: string;
  focal_length_mm: number;
  aperture_fstop: number; // e.g. 1.4, 2.8, 5.6
  focus_distance_m: number;
  shutter_angle_deg: number; // e.g. 180
  iso_rating: number; // e.g. 800
}

export interface LightingRigConfigView {
  key_light_intensity: number; // 0.0 to 1.0
  fill_light_intensity: number;
  back_light_intensity: number;
  color_temperature_kelvin: number; // 2700K - 6500K
}

// React Feature Components
export declare const CinematographyWorkspace: React.FC;
export declare const LensProfilePanel: React.FC;
export declare const CameraSettingsPanel: React.FC;
export declare const LightingControlPanel: React.FC;
export declare const DepthOfFieldVisualizer: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 37 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── cinematography.json
            │       └── en-US/
            │           └── cinematography.json
            └── features/
                └── cinematography/ # Cinematography Studio feature package
                    ├── types.ts    # Optics & lighting UI models
                    ├── CinematographyWorkspace.tsx # Master optics workspace
                    ├── LensProfilePanel.tsx       # Lens profile selector
                    ├── CameraSettingsPanel.tsx    # F-stop, ISO, shutter controls
                    ├── LightingControlPanel.tsx   # Three-point lighting & Kelvin
                    └── DepthOfFieldVisualizer.tsx # DoF focus range visualizer
```

---

## 6. Testing & Validation Strategy

1. **Camera Optics Parameter Test**: Adjust aperture to `f/1.4`; verify DoF visualizer recalculates shallow depth of field.
2. **Lighting Control IPC Test**: Adjust Kelvin color temperature slider; verify IPC payload emits `cinematography_update_lighting` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`ஒளிப்பதிவுத் துறை`).

---

## 7. Acceptance Criteria

Module 37 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Cinematography Studio components render optics controls, DoF visualizers, and lighting panels cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all optics controls.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 37: Cinematography Studio UI**.
> 2. Upon your explicit approval, I will execute Module 37 implementation (`apps/studio-ui/src/features/cinematography/`).
