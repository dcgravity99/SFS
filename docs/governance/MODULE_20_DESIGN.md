# MODULE 20 DESIGN SPECIFICATION: SCENE ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 20 establishes the **Scene Engine** (`sira-engine-scene`) for **Siragugal Film Studio**. It implements spatial environment layout setup, 3D camera bounding volumes, character positioning matrices, prop placement registries, and spatial scene graph coordinates specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Spatial Environment Coordinator**: Index 3D scene environment boundaries, stage origin points, and coordinate transform matrices.
2. **3D Camera Bounding Volumes**: Define camera FOV cones, frustum bounds, and spatial safety buffers preventing camera-actor clipping.
3. **Character & Prop Position Matrix**: Position characters and props within 3D spatial scene grids (`x, y, z, pitch, yaw, roll`).
4. **Spatial Occlusion & Collision Verifier**: Validate spatial layouts to ensure props and actors do not intersect or block camera sightlines unexpectedly.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 19 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 19](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_19_COMPLETION.md).

---

## 4. Public Interfaces

Module 20 exposes public scene engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_scene)
pub struct SceneEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform3D {
    pub position: [f32; 3],  // x, y, z
    pub rotation: [f32; 3],  // pitch, yaw, roll
    pub scale: [f32; 3],     // sx, sy, sz
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialSceneNode {
    pub node_id: String,
    pub entity_type: String, // Character, Prop, Camera, Light
    pub transform: Transform3D,
    pub bounding_radius_meters: f32,
}

impl SceneEngine {
    pub fn create_scene_layout(scene_id: &str) -> SiraResult<String>;
    pub fn place_entity(scene_id: &str, node: SpatialSceneNode) -> SiraResult<()>;
    pub fn verify_spatial_collisions(scene_id: &str) -> SiraResult<bool>; // Returns true if layout is collision-free
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 20 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-scene/           # Rust Scene Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & SceneEngine API
            ├── layout.rs           # SpatialSceneNode & 3D layout coordinator
            ├── camera_bounds.rs    # Camera FOV & frustum bounding volumes
            ├── props.rs            # Prop & asset placement registry
            └── occlusion.rs        # Spatial occlusion & collision verifier
```

---

## 6. Testing & Validation Strategy

1. **Spatial Layout Creation Test**: Create 3D scene layout; verify origin transform defaults to identity `[0, 0, 0]`.
2. **Entity Placement Test**: Place character and prop node; verify spatial node registry records transforms accurately.
3. **Collision Detector Test**: Place two overlapping nodes (distance < combined radius); verify collision detector flags intersection.

---

## 7. Acceptance Criteria

Module 20 is accepted when:
1. `packages/sira-engine-scene` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. 3D spatial scene layout setup, node placement, and collision checks pass 100% of unit tests.
3. Zero UI or AI video generation feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 20: Scene Engine**.
> 2. Upon your explicit approval, I will execute Module 20 implementation (`packages/sira-engine-scene`).
