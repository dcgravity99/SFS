# MODULE 00: ENGINEERING FOUNDATION & GOVERNANCE STANDARDS
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED GOVERNANCE SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Purpose & Scope

Module 00 establishes the permanent engineering, governance, code style, security, and quality standards for **Siragugal Film Studio**. All subsystems, packages, contributors, and subagents must strictly comply with these standards.

---

## 2. Naming Conventions

### 2.1 Repository & Directory Naming
- **Repository Name**: `SiragugalFilmStudio`
- **Root Directory Layout**: Lowercase hyphenated or single-word directories (`apps/`, `packages/`, `plugins/`, `docs/`, `tools/`).
- **Sub-directories**: Lowercase kebab-case (`packages/sira-core/`, `packages/media-pipeline/`).

### 2.2 File & Module Naming
- **Rust Files & Modules**: Lowercase snake_case (`vram_allocator.rs`, `media_pipeline.rs`).
- **TypeScript / React Files**: PascalCase for React Components (`TimelineView.tsx`), camelCase for utilities (`timecodeFormat.ts`).
- **C++ Files**: Lowercase snake_case (`hal_cuda_device.cpp`, `hal_metal_device.mm`).
- **Documentation Files**: Uppercase snake_case for top-level governance (`CONSTITUTION.md`, `ENGINEERING_FOUNDATION.md`), lowercase snake_case for technical specs (`system_architecture.md`).

### 2.3 Package & Namespace Conventions
- **TypeScript NPM Packages**: `@sira/<package-name>` (e.g. `@sira/core-types`, `@sira/sfsp-engine`).
- **Rust Cargo Crates**: `sira_<crate_name>` (e.g. `sira_core`, `sira_hal`, `sira_config`).
- **C++ Namespaces**: `sira::<subsystem>` (e.g. `sira::hal`, `sira::media`).

---

## 3. Version Control & Git Strategy

### 3.1 Branching Model
- `main`: Production releases only. Protected branch requiring 100% passing tests and Architect sign-off.
- `develop`: Nightly integration branch.
- `feature/<module-name>`: Isolated feature branches matching specific modules or ADRs (e.g. `feature/m01-monorepo-setup`).
- `hotfix/<issue-id>`: Critical patch releases.

### 3.2 Commit Message Convention (Conventional Commits)
All commit messages must follow: `<type>(<scope>): <short description>`
- `feat`: New feature or module addition.
- `fix`: Bug fix.
- `docs`: Documentation update.
- `style`: Formatting, missing semi-colons, linting fixes.
- `refactor`: Code change that neither fixes a bug nor adds a feature.
- `test`: Adding missing tests or correcting existing tests.
- `chore`: Build system, toolchain, or dependency updates.

---

## 4. Licensing & Code Ownership

- **Open Source License**: Dual Apache 2.0 / MIT License.
- **License Header**: Mandatory header on every source code file (`.rs`, `.ts`, `.tsx`, `.cpp`, `.h`):

```cpp
/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 *
 * Licensed under the Apache License, Version 2.0 or the MIT License.
 * See LICENSE files in project root for full terms.
 * ============================================================================ */
```

---

## 5. Dependency Governance

1. **Approved Dependency Sources**: Official registries only (`crates.io` for Rust, `npmjs.com` for TS, official distribution tarballs for C++). Direct git dependency links are forbidden in release builds.
2. **Vulnerability Scanning**: Automated `cargo audit` and `pnpm audit` run on every CI build. Critical/High vulnerabilities block release builds.
3. **SBOM Generation**: Software Bill of Materials (SBOM) generated in CycloneDX JSON format during release builds.
4. **Dependency Update Policy**: Monthly routine minor dependency updates; emergency security updates merged within 24 hours.

---

## 6. API Stability Policy

| Stability Lifecycle Stage | Definition & SemVer Guarantee |
| :--- | :--- |
| **Experimental** | Subject to breaking changes without notice. Internal or prototype APIs (`@experimental`). |
| **Preview** | Feature-complete APIs under community evaluation; breaking changes require 1 minor version notice. |
| **Stable** | Production-ready APIs. No breaking changes permitted without major SemVer bump (`v1.0` -> `v2.0`). |
| **Deprecated** | Active but scheduled for removal in next major release. Triggers compiler/linter warnings. |
| **Removed** | Completely purged from codebase following deprecation lifecycle. |

---

## 7. Configuration Hierarchy

Configuration resolution follows strict order of precedence (higher numbers override lower numbers):

```
1. Built-in Code Defaults (Lowest Priority)
   ↓
2. System Configuration File (/etc/sira/studio.json or C:\ProgramData\Sira\studio.json)
   ↓
3. User Configuration File (~/.config/sira/studio.json)
   ↓
4. Project Configuration File (.sfsp/manifest.json)
   ↓
5. Environment Variables (SIRA_*)
   ↓
6. Command Line Arguments (--vram-limit, --config) (Highest Priority)
```

---

## 8. Error Code Standard

Errors across all languages must emit structured error objects containing a unique numeric code matching subsystem ranges:

| Error Code Range | Subsystem Domain | Example Error Code |
| :--- | :--- | :--- |
| **SIRA-1000 to 1999** | System Core & Workspace Engine | `SIRA-1001: WORKSPACE_INITIALIZATION_FAILED` |
| **SIRA-2000 to 2999** | Hardware Abstraction Layer (HAL) | `SIRA-2015: CUDA_VRAM_ALLOCATION_OOM` |
| **SIRA-3000 to 3999** | SIRA AI Core & Model Registry | `SIRA-3008: MODEL_CHECKSUM_VERIFICATION_FAILED` |
| **SIRA-4000 to 4999** | Project `.sfsp` Engine & Asset DB | `SIRA-4002: SFSP_MANIFEST_CORRUPTED` |
| **SIRA-5000 to 5999** | Workflow Graph Engine | `SIRA-5012: WORKFLOW_DAG_CYCLE_DETECTED` |
| **SIRA-6000 to 6999** | Plugin Runtime & WASM Sandbox | `SIRA-6004: PLUGIN_PERMISSION_DENIED` |
| **SIRA-7000 to 7999** | Render Scheduler & Resource Manager | `SIRA-7009: RENDER_CHECKPOINT_RESUME_FAILED` |

---

## 9. Logging Standard

- **Log Levels**: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `FATAL`.
- **Structured JSON Schema**:
  ```json
  {
    "timestamp": "2026-08-03T09:42:30.123Z",
    "level": "INFO",
    "subsystem": "sira-hal",
    "error_code": null,
    "message": "Initialized Metal compute device",
    "correlation_id": "corr-8f921a",
    "job_id": "job-1042",
    "context": { "device_name": "Apple M3 Max", "vram_mb": 36864 }
  }
  ```
- **Correlation & Job IDs**: Passed across thread boundaries and IPC channels to trace multi-step AI tasks.

---

## 10. Core Coding Principles

1. **Composition over Inheritance**: Favor traits/interfaces and component composition over deep class hierarchies.
2. **Low Coupling & High Cohesion**: Modules operate through narrow, explicit API contracts.
3. **Dependency Injection**: Pass dependencies (allocators, mock providers, loggers) explicitly to facilitate unit testing.
4. **No Cyclic Dependencies**: Dependencies flow strictly downward in the architectural hierarchy; cyclic dependencies fail build compilation.

---

## 11. ADR Numbering & Domain Assignment Policy

ADRs are indexed sequentially using assigned domain ranges:

| ADR Range | Domain Area |
| :--- | :--- |
| **ADR 0001 - 0099** | Core Architecture, Governance & Strategy |
| **ADR 0100 - 0199** | SIRA AI Core, Model Registry & Workflows |
| **ADR 0200 - 0299** | Hardware Abstraction Layer (HAL) & Compute |
| **ADR 0300 - 0399** | Project Format (`.sfsp`), Asset DB & Undo |
| **ADR 0400 - 0499** | Render Scheduler & Resource Management |
| **ADR 0500 - 0599** | Plugin SDK & Marketplace Architecture |

---

## 12. Release Channel Policy

- **Nightly**: Automated daily builds from `develop` branch for active testing.
- **Preview**: Bi-weekly pre-release builds containing feature-complete modules under community evaluation.
- **Stable**: Fully validated production releases from `main` branch.
- **LTS (Long Term Support)**: Designated stable releases supported with security patches for 24 months.

---

## 13. Performance Budgets

Target engineering budgets for core platform infrastructure:

| Infrastructure Metric | Performance Target Budget |
| :--- | :--- |
| **Application Startup Time** | < 1.5 seconds to interactive desktop shell |
| **IPC Roundtrip Latency** | < 2.0 milliseconds for gRPC control messages |
| **Shared Memory Frame Throughput** | > 120 FPS at 4K resolution (3840x2160 uncompressed) |
| **Project Load Time (`.sfsp`)** | < 500 milliseconds for standard 100MB project |
| **VRAM Allocator Overhead** | < 15 milliseconds for weight swap allocation |
| **WASM Plugin Invoke Latency** | < 0.5 milliseconds per node invocation |

---

## 14. Contributor Review & PR Checklist

Every pull request must satisfy the 6-point checklist before merge:

- [ ] **Architecture**: Verified compliance with Constitution v1.2.0 and ADRs.
- [ ] **Documentation**: Public API docstrings updated (`rustdoc` / `JSDoc`).
- [ ] **Testing**: 80%+ unit coverage; tests pass on macOS & Windows.
- [ ] **Security**: No hardcoded keys, sanitized input paths, permissions checked.
- [ ] **Performance**: Passes defined performance budgets; zero memory leaks.
- [ ] **Licensing**: Approved license header present on all new source files.

---

## 15. Definition of Ready (DoR) & Definition of Done (DoD)

### Definition of Ready (DoR)
A module is Ready to implement when:
1. Module design document produced detailing Purpose, Dependencies, Public Interfaces, Testing Strategy, and Acceptance Criteria.
2. Design document submitted and explicitly APPROVED by Project Owner.
3. Architecture alignment verified against Constitution v1.2.0.

### Definition of Done (DoD)
A module is Done when:
1. Production code written cleanly with zero clippy/eslint warnings.
2. 100% of unit & integration tests pass on macOS & Windows.
3. Public API documentation generated and updated.
4. Security checklist passed.
5. Final review completed and explicitly APPROVED by Project Owner.
