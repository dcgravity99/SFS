# MODULE 35 DESIGN SPECIFICATION: SCENE BUILDER UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 35 establishes the **Scene Builder UI** (`apps/studio-ui/src/features/scene/`) for **Siragugal Film Studio**. It implements 3D spatial scene graph inspector panels, camera node placement controls, prop asset registry pickers (`AssetId` handles), marker transform gizmos (Position X/Y/Z, Rotation, Scale), frustum occlusion validators, and live IPC integration with `sira_engine_scene` (Module 20) without adding unapproved 3D rendering engines or AI video generation features.

---

## 2. Module Responsibilities & Core Features

1. **Spatial Scene Graph Tree Inspector**: Hierarchy panel displaying `SceneGraph` nodes (`CameraNode`, `CharacterNode`, `PropNode`, `MarkerNode`, `EnvironmentNode`).
2. **Transform Gizmo & Coordinate Inspector**: Position X/Y/Z, Rotation Roll/Pitch/Yaw, and Scale inspector for active scene nodes.
3. **Prop Asset Registry Picker**: Searchable prop library panel binding `AssetId` model handles stored in `asset_db` to 3D scene nodes.
4. **Camera Occlusion & Frustum Validator**: Visual indicator panel highlighting camera placement occlusion or bounding box collision warnings.
5. **Sub-Engine IPC Integration**: Send scene graph updates to `sira_engine_scene` via `StudioIpcService.executeEngineCommand('scene_add_node', ...)` and `scene_validate_occlusion`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 20 (`sira_engine_scene`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 34 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_34_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 35 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/scene/types.ts)
export interface SceneTransformView {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface SceneNodeView {
  node_id: string;
  name: string;
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
2. **Transform Inspector Update Test**: Adjust Position X slider; verify transform state updates and emits `scene_update_transform` IPC command.
3. **Prop Binding Integration Test**: Bind `AssetId` reference; verify node updates cleanly with zero filesystem path exposure.

---

## 7. Acceptance Criteria

Module 35 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Scene graph inspector renders node trees and transforms cleanly.
3. IPC commands communicate with `sira_engine_scene` with WCAG 2.2 AA accessibility support.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 35: Scene Builder UI**.
> 2. Upon your explicit approval, I will execute Module 35 implementation (`apps/studio-ui/src/features/scene/`).
