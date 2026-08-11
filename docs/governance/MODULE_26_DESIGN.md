# MODULE 26 DESIGN SPECIFICATION: ASSET PIPELINE ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 26 establishes the **Asset Pipeline Engine** (`sira-engine-asset`) for **Siragugal Film Studio**. It implements multi-format raw media ingestion (4K/8K RAW video, EXR image sequences, WAV audio), background proxy video generation (1080p / 720p editing proxies), media format transcoding, SHA-256 asset checksum verification, and `asset_db` indexing specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or creative media rendering logic.

---

## 2. Module Responsibilities & Core Features

1. **Multi-Format Media Ingest**: Ingest raw video, image, audio, 3D mesh (`.gltf`/`.usd`), and text script assets into `.sfsp` project bundles.
2. **Proxy Video Generator**: Generate lightweight 720p / 1080p ProRes Proxy or H.264 video editing proxies for smooth timeline scrubbing.
3. **SHA-256 Asset Integrity Verifier**: Compute and verify SHA-256 hashes during asset ingest and cache transfers to prevent file corruption (`SIRA-4002`).
4. **Media Transcoding Pipeline**: Transcode incompatible video and audio formats to studio standard mezzanine formats (ProRes / Linear PCM WAV).

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 25 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `sira_engine_render`, `resource_manager`, `cache_manager`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 25](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_25_COMPLETION.md).

---

## 4. Public Interfaces

Module 26 exposes public asset pipeline engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_asset)
pub struct AssetPipelineEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestJobSpec {
    pub source_path: String,
    pub asset_type: String, // Video, Audio, Image, Mesh, Script
    pub create_proxy: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestResult {
    pub asset_id: String,
    pub sha256_checksum: String,
    pub proxy_path: Option<String>,
    pub file_size_bytes: u64,
}

impl AssetPipelineEngine {
    pub fn ingest_asset(spec: IngestJobSpec) -> SiraResult<IngestResult>;
    pub fn generate_proxy(asset_id: &str, target_resolution: &str) -> SiraResult<String>;
    pub fn verify_checksum(asset_id: &str, expected_hash: &str) -> SiraResult<bool>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 26 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-asset/           # Rust Asset Pipeline Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & AssetPipelineEngine API
            ├── ingest.rs           # Multi-format media ingest coordinator
            ├── proxy.rs            # Proxy video generator (720p / 1080p)
            ├── checksum.rs         # SHA-256 asset integrity verifier
            └── transcode.rs        # Media format transcoding pipeline
```

---

## 6. Testing & Validation Strategy

1. **Asset Ingest & Checksum Test**: Ingest test video file; verify `AssetId` UUID v7 is generated and SHA-256 hash matches expected digest.
2. **Proxy Video Generation Test**: Generate 720p proxy; verify proxy file path is recorded in `asset_db`.
3. **Checksum Failure Test**: Supply tampered file; verify SHA-256 verifier returns hash mismatch failure.

---

## 7. Acceptance Criteria

Module 26 is accepted when:
1. `packages/sira-engine-asset` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Asset ingestion, proxy video generation, and SHA-256 checksum verification pass 100% of unit tests.
3. Zero UI or application feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 26: Asset Pipeline Engine**.
> 2. Upon your explicit approval, I will execute Module 26 implementation (`packages/sira-engine-asset`).
