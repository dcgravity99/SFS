# MODULE 03 DESIGN SPECIFICATION: CORE LIBRARIES & SHARED PACKAGES
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 03 establishes the fundamental shared core libraries, SMPTE timecode calculation primitives, strongly typed UUID v7 identifiers, structured error types (`SIRA-1000` to `SIRA-7999`), rich async job result wrappers, feature flag abstractions, and serialization rules for **Siragugal Film Studio**.

---

## 2. Module Responsibilities & Core Features

1. **SMPTE Timecode Engine (`SiraTimecode`)**: Support rational frame rates (`24000/1001` NTSC, `24/1` film, `25/1` PAL, `30000/1001` drop-frame, `5994/100`), drop-frame/non-drop-frame string parsing (`HH:MM:SS;FF` vs `HH:MM:SS:FF`), integer frame count conversions, and frame math.
2. **Strongly Typed Identifiers**: Provide domain-specific UUID v7 wrappers (`ProjectId`, `SceneId`, `AssetId`, `CharacterId`, `WorkflowId`, `RenderJobId`) to prevent raw string ID confusion.
3. **Structured SiraError Schema**: Include numeric code, category, severity (`Fatal`, `Error`, `Warning`), recoverability boolean, correlation ID, job ID, context map, and localization key (`i18n_key`).
4. **Rich Async SiraResult<T>**: Support success, error, progress percentage (0.0-1.0), cancellation state, validation failure list, and partial success.
5. **Layered Validation Strategy**: Enforce 4-tier validation (Input syntax, Domain schema, Business invariants, Cross-module contracts).
6. **Feature Flag Manager**: Thread-safe feature toggle registry (`FeatureFlagManager`) for experimental capabilities.
7. **Localization Readiness**: Core libraries strictly avoid hardcoded human-readable strings, returning structured `i18n_key` symbols.
8. **Binary & API Compatibility**: Zero platform-specific, UI, AI provider, or render engine dependencies.

---

## 3. SMPTE Timecode & Rational Frame Rate Matrix

| Frame Rate Alias | Rational Representation | Drop-Frame Support | Standard Timecode Format |
| :--- | :--- | :--- | :--- |
| **23.976 fps (NTSC Film)** | `24000 / 1001` | Non-Drop Frame | `HH:MM:SS:FF` |
| **24.0 fps (Standard Film)** | `24 / 1` | Non-Drop Frame | `HH:MM:SS:FF` |
| **25.0 fps (PAL / SECAM)** | `25 / 1` | Non-Drop Frame | `HH:MM:SS:FF` |
| **29.97 fps (NTSC Video)** | `30000 / 1001` | Drop-Frame (`HH:MM:SS;FF`) & Non-Drop | `HH:MM:SS;FF` |
| **59.94 fps (High Frame Rate)** | `60000 / 1001` | Drop-Frame & Non-Drop | `HH:MM:SS;FF` |

---

## 4. Structured Error Schema (`SiraError`)

```json
{
  "code": 2015,
  "error_name": "CUDA_VRAM_ALLOCATION_OOM",
  "category": "HARDWARE_ABSTRACTION",
  "severity": "ERROR",
  "is_recoverable": true,
  "correlation_id": "corr-98124",
  "job_id": "job-4012",
  "i18n_key": "errors.hal.vram_oom",
  "context": {
    "requested_mb": 8192,
    "available_mb": 2048
  },
  "suggested_action": "i18n.actions.unload_idle_models"
}
```

---

## 5. File Blueprint

Module 03 implements the following core package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    ├── core-types/                 # TypeScript shared package
    │   ├── package.json
    │   ├── tsconfig.json
    │   └── src/
    │       ├── index.ts            # Root export
    │       ├── timecode.ts         # SMPTE timecode & drop-frame parser
    │       ├── errors.ts           # SiraError & error code constants
    │       ├── ids.ts              # Strongly typed UUID v7 wrappers
    │       ├── results.ts          # SiraResult, progress & async job status
    │       └── feature_flags.ts    # FeatureFlagManager abstraction
    └── sira-types/                 # Rust shared crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Root export
            ├── timecode.rs         # SMPTE timecode & drop-frame implementation
            ├── errors.rs           # SiraError struct & code constants
            ├── ids.rs              # Strongly typed UUID v7 structs
            ├── results.rs          # SiraResult enum & job state
            └── feature_flags.rs    # FeatureFlagManager thread-safe registry
```

---

## 6. Acceptance Criteria

Module 03 is accepted when:
1. `packages/core-types/` and `packages/sira-types/` compile with zero warnings across TypeScript and Rust.
2. SMPTE drop-frame and non-drop-frame timecode calculations pass 100% of unit test cases.
3. Strongly typed identifiers (`ProjectId`, `SceneId`, etc.) and `SiraError` instances serialize/deserialize roundtrip identically between Rust and TypeScript.
4. Core packages remain 100% independent of UI, AI providers, rendering engines, and platform-specific code.
5. Zero application or creative feature code is present.
