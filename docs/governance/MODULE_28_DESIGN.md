# MODULE 28 DESIGN SPECIFICATION: PROJECT PACKAGING ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 28 establishes the **Project Packaging Engine** (`sira-engine-packaging`) for **Siragugal Film Studio**. It implements `.sfsp` (Siragugal Film Studio Package) binary project archive creation, zstd/zip compression, Ed25519 digital signature signing and verification, `sfsp_engine` manifest index validation (`SIRA-4002`), asset dependency bundle packing, and archive extraction specified in [docs/governance/PHASE_2_MASTER_PLAN.md](file:///D:/SiragugalFilmStudio/docs/governance/PHASE_2_MASTER_PLAN.md) without adding UI views or application feature logic.

---

## 2. Module Responsibilities & Core Features

1. **`.sfsp` Project Archive Bundler**: Package project assets (`project.db`, `cache.db`, screenplays, audio tracks, models, renders) into standalone versioned `.sfsp` archives.
2. **Ed25519 Digital Signature Verifier**: Sign and verify project archive integrity using Ed25519 cryptographic signatures.
3. **High-Ratio Lossless Compression**: Compress project assets using zstd / ZIP compression engines to reduce project file footprint.
4. **Manifest & Asset Index Validator**: Validate `manifest.json` schema and verify file checksums during package unbundling (`SIRA-4002`).

---

## 3. Module Dependencies

- **Software Dependencies**: Modules 01 - 27 (`sira_types`, `sira_config`, `sira_diagnostics`, `sfsp_engine`, `asset_db`, `sira_hal`, `sira_core`, `sira_ai_provider`, `workflow_engine`, `experience_layer`, `sira_engine_story`, `sira_engine_character`, `sira_engine_actor`, `sira_engine_scene`, `sira_engine_director`, `sira_engine_cinematography`, `sira_engine_audio`, `sira_engine_timeline`, `sira_engine_render`, `sira_engine_asset`, `sira_engine_workflow`, `resource_manager`, `cache_manager`), Rust `serde_json`.
- **Module Dependencies**: Depends on [Modules 01 - 27](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_27_COMPLETION.md).

---

## 4. Public Interfaces

Module 28 exposes public project packaging engine interfaces across Rust:

```rust
// Rust Public Interface (sira_engine_packaging)
pub struct ProjectPackagingEngine;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageSpec {
    pub project_path: String,
    pub output_sfsp_path: String,
    pub compression_level: i32, // 1 to 22 (zstd)
    pub sign_archive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub sfsp_version: String,
    pub project_name: String,
    pub total_assets_count: usize,
    pub archive_size_bytes: u64,
    pub ed25519_signature: Option<String>,
}

impl ProjectPackagingEngine {
    pub fn create_package(spec: PackageSpec) -> SiraResult<PackageMetadata>;
    pub fn extract_package(sfsp_path: &str, destination_dir: &str) -> SiraResult<PackageMetadata>;
    pub fn verify_package_signature(sfsp_path: &str, public_key_hex: &str) -> SiraResult<bool>;
}
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 28 will create the following package structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-engine-packaging/      # Rust Project Packaging Engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & ProjectPackagingEngine API
            ├── bundler.rs          # .sfsp binary project archive bundler
            ├── compression.rs      # zstd / ZIP lossless compression engine
            ├── signature.rs        # Ed25519 digital signature signer & verifier
            └── validator.rs        # sfsp_engine manifest & asset index validator
```

---

## 6. Testing & Validation Strategy

1. **Package Creation & Compression Test**: Bundle sample project directory; verify `.sfsp` archive is created and compressed cleanly.
2. **Ed25519 Signature Verification Test**: Sign package; verify valid signature returns true; verify tampered package signature returns false.
3. **Archive Extraction Test**: Extract `.sfsp` package to destination directory; verify `manifest.json` and asset files extract intact.

---

## 7. Acceptance Criteria

Module 28 is accepted when:
1. `packages/sira-engine-packaging` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Project bundling, Ed25519 signature verification, and extraction pass 100% of unit tests.
3. Zero UI or application feature code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 28: Project Packaging Engine**.
> 2. Upon your explicit approval, I will execute Module 28 implementation (`packages/sira-engine-packaging`).
