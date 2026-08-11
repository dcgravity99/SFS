# MODULE 01 DESIGN SPECIFICATION: MONOREPO & WORKSPACE SETUP
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 01 establishes the multi-language monorepo workspace structure for **Siragugal Film Studio**. It defines package boundaries, toolchain integration, shared scripts, dependency managers (`pnpm` for Node/TypeScript and `cargo` for Rust), developer container support, CI pipelines, and repository community governance without adding production application code.

---

## 2. Module Responsibilities

1. **Workspace Boundary Definition**: Initialize root `pnpm-workspace.yaml` and root Rust `Cargo.toml` workspace.
2. **Toolchain Version Pinning**: Pin Node.js (`.nvmrc`), Rust (`rust-toolchain.toml`), and `pnpm` (`package.json` engines).
3. **Directory Skeleton Initialization**: Create root directory boundaries (`apps/`, `packages/`, `plugins/`, `docs/`, `tools/`, `.github/`, `.devcontainer/`).
4. **Linting, Formatting & Editor Rules**: Provide root configurations for `.editorconfig`, `.prettierrc`, `.eslintrc.js`, `rustfmt.toml`, `clippy`, and `.clang-format`.
5. **Git, Hooks & Security Scanning**: Establish `.gitignore`, `.gitattributes`, pre-commit hooks (secret scanning, license header verification, large-file detection, Conventional Commits), and license files (`LICENSE-APACHE`, `LICENSE-MIT`).
6. **Community & Governance Files**: Include `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `SUPPORT.md`, `CHANGELOG.md`, `.github/PULL_REQUEST_TEMPLATE.md`, and issue templates.
7. **Task Automation & Dev Containers**: Provide `.devcontainer/devcontainer.json` and cross-platform setup script (`tools/scripts/bootstrap.js`).
8. **CI Pipeline Definition**: Configure multi-stage GitHub Actions CI (`Lint` → `Test` → `Build` → `Security Scan` → `Artifact Packaging`).

---

## 3. Toolchain Pinning & Version Management

- **Node.js**: `20.11.1` (Pinned in `.nvmrc` & `package.json` engines: `>=20.0.0`).
- **pnpm**: `9.0.0` (Pinned in `package.json` packageManager: `pnpm@9.0.0`).
- **Rust Toolchain**: `1.76.0` (Pinned in `rust-toolchain.toml` with `clippy`, `rustfmt`, `llvm-tools` components).
- **C++ Compiler**: Clang 16+ or MSVC 2022+ (C++20 standard).

---

## 4. CI Pipeline & Pre-Commit Hook Architecture

### 4.1 CI Pipeline Stages (`.github/workflows/ci.yml`)
1. **Stage 1: Lint & Format**: Run `prettier`, `eslint`, `rustfmt`, `clippy -- -D warnings`, `clang-format`.
2. **Stage 2: Test Harness**: Run `cargo test --workspace` and `pnpm test` on macOS & Windows.
3. **Stage 3: Security & SBOM**: Run `cargo audit`, `pnpm audit`, secret scanning, and CycloneDX SBOM generation.
4. **Stage 4: Workspace Build**: Validate zero-error compilation of workspace targets.
5. **Stage 5: Packaging Verification**: Validate installer packaging integrity.

### 4.2 Pre-Commit Hook Protections
- **Secret Scanning**: Scans diffs for leaked API keys, tokens, or private keys.
- **License Header Verification**: Checks that new source files contain the mandatory Apache/MIT copyright header.
- **Large-File Detection**: Rejects tracking binary files > 10MB without Git LFS.
- **Commit-Msg Hook**: Enforces Conventional Commits specification (`feat`, `fix`, `docs`, etc.).

---

## 5. File Blueprint

Module 01 creates the following file structure:

```
D:\SiragugalFilmStudio\
├── README.md                       # Community Overview, Setup Guide & Badge Placeholders
├── CONTRIBUTING.md                 # Contributor Workflow & Pull Request Guide
├── CODE_OF_CONDUCT.md              # Contributor Covenant Code of Conduct v2.1
├── SECURITY.md                     # Security Vulnerability Reporting Policy
├── SUPPORT.md                      # Community Support Channels & Resources
├── CHANGELOG.md                    # Keep a Changelog v1.1.0 Version History
├── .gitignore                      # Git ignore rules
├── .gitattributes                  # LF line-ending normalization
├── .editorconfig                   # Editor whitespace & indentation rules
├── .nvmrc                          # Node.js version pin (20.11.1)
├── rust-toolchain.toml             # Rust channel & component pin (1.76.0)
├── LICENSE-APACHE                  # Apache 2.0 License
├── LICENSE-MIT                     # MIT License
├── package.json                    # Root pnpm package config & master script runner
├── pnpm-workspace.yaml             # pnpm workspace definition
├── Cargo.toml                      # Root Rust workspace definition
├── .prettierrc                     # Code formatting rules
├── .eslintrc.js                    # ESLint rule configuration
├── rustfmt.toml                    # Rust formatting rules
├── .clang-format                   # C++ formatting rules
├── .devcontainer/
│   └── devcontainer.json           # VS Code Dev Container environment config
├── .github/
│   ├── PULL_REQUEST_TEMPLATE.md    # Pull Request Audit Checklist Template
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md           # Bug Report Issue Template
│   │   └── feature_request.md      # Feature Request Issue Template
│   └── workflows/
│       └── ci.yml                  # GitHub Actions CI multi-stage workflow
└── tools/
    └── scripts/
        └── bootstrap.js            # Cross-platform developer setup script
```

---

## 6. Build Reproducibility Expectations

1. **Locked Lockfiles**: All dependency installations MUST use locked versions (`pnpm-lock.yaml` & `Cargo.lock`). Floating dependency tags (`^`, `~`, `latest`) are strictly forbidden in `package.json` for production dependencies.
2. **Deterministic Output**: Build toolchain flags enforce deterministic binary generation across identical OS targets.

---

## 7. Acceptance Criteria

Module 01 is accepted when:
1. All 26 blueprint files are generated and committed cleanly.
2. A fresh clone of the repository bootstraps successfully using `node tools/scripts/bootstrap.js` following only documented README instructions.
3. Root `pnpm-workspace.yaml` and `Cargo.toml` resolve workspace boundaries without errors.
4. `pnpm install` and `cargo check --workspace` pass cleanly.
5. All pre-commit hooks, secret scanning, and license verification scripts pass.
6. Zero application or creative feature code is present.
