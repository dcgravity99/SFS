# MODULE 35 DESIGN SPECIFICATION v2.0: SCENE BUILDER UI (GLOBALIZATION & LOCALIZATION INTEGRATED)
**Siragugal Film Studio**  
**Document Version**: 2.0.0  
**Status**: REVISED DESIGN SPECIFICATION FOR REVIEW  
**Author**: AG (Permanent Chief Software Architect)  

---

## 1. Executive Summary

Module 35 establishes the **Scene Builder UI** (`apps/studio-ui/src/features/scene/`) for **Siragugal Film Studio**. Version 2.0 incorporates the mandatory **Globalization & Localization Architecture Requirement**, establishing Siragugal Film Studio as a **Tamil-first (`ta-IN`)** professional filmmaking platform with **English (`en-US`)** secondary fallback.

---

## 2. Module Responsibilities & Core Features

1. **Spatial Scene Graph Tree Inspector**: Hierarchy panel displaying `SceneGraph` nodes (`CameraNode`, `CharacterNode`, `PropNode`, `MarkerNode`, `EnvironmentNode`).
2. **Transform Gizmo & Coordinate Inspector**: Position X/Y/Z, Rotation Roll/Pitch/Yaw, and Scale inspector for active scene nodes.
3. **Prop Asset Registry Picker**: Searchable prop library panel binding `AssetId` model handles stored in `asset_db` to 3D scene nodes.
4. **Camera Occlusion & Frustum Validator**: Visual indicator panel highlighting camera placement occlusion or bounding box collision warnings.
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization, font fallbacks (`Noto Sans Tamil`, `Inter`), and localized scene metadata mappings.
6. **Sub-Engine IPC Integration**: Send scene graph updates to `sira_engine_scene` via `StudioIpcService.executeEngineCommand('scene_add_node', ...)` and `scene_validate_occlusion`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 20 (`sira_engine_scene`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), i18next / React-i18next, Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 34 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_34_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 35 exposes the following React components and TypeScript models:

```typescript
// Localized Text Map Interface
export interface LocalizedTextMap {
  "ta-IN": string; // Tamil (Primary)
  "en-US": string; // English (Secondary Fallback)
}

export interface SceneTransformView {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface SceneNodeView {
  node_id: string; // Machine-readable UUIDv7
  display_name: LocalizedTextMap;
  node_type: 'Camera' | 'Character' | 'Prop' | 'Marker' | 'Environment';
  transform: SceneTransformView;
  asset_id?: string;
}

// React Feature Components
export declare const SceneGraphInspector: React.FC;
export declare const TransformInspector: React.FC;
export declare const PropRegistryPicker: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 35 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   ├── common.json
            │       │   └── scene.json
            │       └── en-US/
            │           ├── common.json
            │           └── scene.json
            └── features/
                └── scene/          # Scene Builder feature package
                    ├── types.ts    # Scene node & transform UI models
                    ├── SceneGraphInspector.tsx # Hierarchical scene graph tree
                    ├── TransformInspector.tsx  # Node position/rotation inspector
                    └── PropRegistryPicker.tsx  # Prop asset registry picker
```

---

## 6. Testing & Validation Strategy

1. **Scene Node Tree Render Test**: Supply `SceneNodeView` hierarchy; verify tree nodes render with typed icons.
2. **Tamil String Externalization Test**: Switch active locale to `ta-IN`; verify UI headers update to Tamil text (`காட்சி அமைப்பு`).
3. **IPC Boundary Test**: Verify IPC command payloads emit machine-readable keys (`node_type: "camera"`), preserving zero localized text in backend IPC contracts.

---

## 7. Performance Targets & Budgets

- **Scene Graph Render Latency**: `< 16 ms` (60 FPS)
- **Locale Switch Delay**: `< 5 ms` (instantaneous UI string swapping)
- **IPC Overhead**: `< 1.0 ms`

---

## 8. Globalization & Localization Compliance

### 8.1 Locale Architecture
- **Primary Locale**: `ta-IN` (Tamil - India)
- **Secondary Locale**: `en-US` (English - United States)
- Architecture accommodates future locale additions (`hi-IN`, `te-IN`, `ja-JP`) without structural code modification.

### 8.2 UI String Externalization
- Zero hardcoded user-facing strings in TSX components.
- All text rendered via localization hook `t("scene.scene_graph")`.

### 8.3 Localization Resource Structure
- `apps/studio-ui/src/i18n/locales/ta-IN/scene.json`:
  ```json
  {
    "scene_graph": "காட்சி அமைப்பு",
    "transform": "மாற்றம்",
    "camera": "கேமரா",
    "position": "நிலை",
    "rotation": "சுழற்சி",
    "scale": "அளவு",
    "asset_picker": "சொத்து தேர்வு"
  }
  ```
- `apps/studio-ui/src/i18n/locales/en-US/scene.json`:
  ```json
  {
    "scene_graph": "Scene Graph",
    "transform": "Transform",
    "camera": "Camera",
    "position": "Position",
    "rotation": "Rotation",
    "scale": "Scale",
    "asset_picker": "Asset Picker"
  }
  ```

### 8.4 Scene Metadata Localization
- Scene objects support `display_name: LocalizedTextMap` while technical identifiers (`node_id`, `node_type`) remain machine-readable.

### 8.5 Asset Localization
- Assets expose `asset_id` + `name: LocalizedTextMap` (`{ asset_id: "asset_001", name: { "ta-IN": "பழைய வீடு", "en-US": "Old House" } }`).

### 8.6 Typography Requirements
- Tamil rendering utilizes `Noto Sans Tamil` with font fallbacks (`Noto Sans Tamil`, `Inter`, `system-ui`).

### 8.7 Accessibility Requirements
- ARIA labels dynamically localized to active locale (`aria-label={t('scene.camera')}`).
- WCAG 2.2 AA compliant contrast and screen reader compatibility.

### 8.8 IPC Localization Boundary
- Backend sub-engines receive machine-readable identifiers (`node_type: "camera"`). IPC contracts contain zero translated strings.

---

## 9. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this revised design specification for **Module 35: Scene Builder UI (v2.0)**.
> 2. Upon your explicit approval, I will proceed with Module 35 implementation (`apps/studio-ui/src/features/scene/`).
