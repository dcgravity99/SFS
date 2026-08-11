# MODULE 22 DESIGN SPECIFICATION: CINEMATOGRAPHY ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 22 establishes the **Cinematography Engine** (`sira-engine-cinematography`) for **Siragugal Film Studio**. It implements camera lens optics calculations (Focal length, Aperture f-stop, Sensor size, Depth of Field), 3D camera motion path generation (Dolly, Crane, Pan, Tilt, Zoom, Rack Focus), lighting grid setup (3-Point Key/Fill/Rim lighting, Color temperature Kelvin), and render camera parameter exports specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or AI video generation logic.

---

## 2. Module Responsibilities & Core Features

1. **Camera Lens Optics Calculator**: Calculate Field of View (FOV angle), hyperfocal distance, circle of confusion, and depth of field (DoF near/far limits).
2. **3D Camera Motion Path Generator**: Generate 3D camera trajectory keyframes along Bezier curves for Dolly, Crane, Pan, Tilt, and Rack Focus moves.
3. **Cinematic Lighting Grid Coordinator**: Define 3-point lighting setups (Key light, Fill light, Backlight/Rim, Key-to-Fill ratio, Kelvin color temperature).
4. **Render Camera Parameter Exporter**: Export camera parameters to `sira_hal` render compute shaders.

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 21 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 21](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_21_COMPLETION.md).

---

## 4. Public Interfaces

Module 22 exposes public cinematography engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_cinematography)
pub struct CinematographyEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraOptics {
    pub focal_length_mm: f32,
    pub aperture_fstop: f32,
    pub sensor_width_mm: f32,
    pub focus_distance_meters: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepthOfField {
    pub fov_degrees: f32,
    pub near_limit_meters: f32,
    pub far_limit_meters: f32,
    pub hyperfocal_distance_meters: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraMotionPath {
    pub motion_type: String, // Dolly, Crane, Pan, Tilt, RackFocus
    pub waypoints: Vec<[f32; 3]>,
    pub duration_seconds: f32,
}

impl CinematographyEngine {
    pub fn compute_optics(optics: &CameraOptics) -> SiraResult<DepthOfField>;
    pub fn generate_motion_path(motion_type: &str, start_pos: [f32; 3], end_pos: [f32; 3], duration: f32) -> SiraResult<CameraMotionPath>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 22 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-cinematography/ # Rust Cinematography Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & CinematographyEngine API
            ├── optics.rs           # Lens optics & Depth of Field calculator
            ├── motion.rs           # 3D Camera motion path generator
            ├── lighting.rs         # 3-Point lighting grid coordinator
            └── exporter.rs         # Render camera parameter exporter
```

---

## 6. Testing & Validation Strategy

1. **Optics & DoF Calculation Test**: Compute depth of field for 50mm lens at f/2.8 focused at 5m; verify near and far limits match optical formula.
2. **Camera Motion Path Generation Test**: Generate Dolly trajectory; verify waypoints interpolate smoothly along duration.
3. **Lighting Grid Setup Test**: Configure 3-point lighting; verify Kelvin color temperature values serialize correctly.

---

## 7. Acceptance Criteria

Module 22 is accepted when:
1. `packages/sira-engine-cinematography` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Lens optics calculation, camera motion path generation, and 3-point lighting grid setup pass 100% of unit tests.
3. Zero UI or AI video generation feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 22: Cinematography Engine**.
> 2. Upon your explicit approval, I will execute Module 22 implementation (`packages/sira-engine-cinematography`).
