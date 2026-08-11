<!--
  Siragugal Film Studio
  Copyright (C) 2026 Siragugal Film Studio Contributors
  Licensed under Apache-2.0 or MIT.
-->

# Siragugal Film Studio 🎬

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#)
[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue)](#)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-informational)](#)
[![Architecture](https://img.shields.io/badge/architecture-v1.2.0%20Frozen-gold)](#)
[![Release](https://img.shields.io/badge/release-v0.1.0--alpha-orange)](#)

> **Official Mission**: "Build a professional, open-source, AI-native filmmaking platform that is modular, extensible, secure, maintainable, and capable of evolving through community contributions."

---

## Overview

**Siragugal Film Studio** is an enterprise-grade, open-source, AI-native desktop filmmaking application built for a 10+ year lifespan. It converts multi-modal human inputs (Voice, Text, Story, Novel, Script, PDF, DOCX, Web, Images) into full-scale cinematic productions, alongside professional AI enhancement tools for video, audio, and stills.

Key Architectural Pillars:
- **Offline-First & AI-First Core**: Local model execution (llama.cpp, Ollama, ComfyUI) with pluggable cloud API fallbacks (user-supplied credentials).
- **Human-in-the-Loop Control**: Non-destructive workflows with persistent Universal Undo across all 14 Creative Studio modules.
- **Hardware Abstraction Layer (HAL)**: Unified GPU compute abstraction over Metal (macOS) and CUDA / DirectML / ROCm / Vulkan (Windows).
- **Native Package Format (`.sfsp`)**: Zero-copy SQLite-backed project container.

---

## Quickstart & Developer Bootstrap

### Prerequisites
- **Node.js**: v20.11.1 LTS (`.nvmrc`)
- **pnpm**: v9.0.0+ (`corepack enable pnpm`)
- **Rust Toolchain**: v1.76.0+ (`rustup toolchain install 1.76.0`)
- **C++ Compiler**: Clang 16+ (macOS) or MSVC 2022+ (Windows)

### Fresh Clone Bootstrap
To bootstrap a fresh clone using only documented instructions, run:

```bash
# Clone the repository
git clone https://github.com/SiragugalFilmStudio/SiragugalFilmStudio.git
cd SiragugalFilmStudio

# Execute cross-platform bootstrap script
node tools/scripts/bootstrap.js
```

The bootstrap script will validate your toolchain, install Node & Rust workspace dependencies, verify configuration files, and setup pre-commit git hooks.

---

## Repository Structure

```
SiragugalFilmStudio/
├── CONSTITUTION.md                 # Project Constitution v1.2.0
├── docs/                           # Engineering Documentation & Architecture Specs
│   ├── architecture/               # Architecture Specs, TDR, Principles, Readiness Audit
│   ├── adr/                        # Architecture Decision Records (0001 - 0006)
│   └── governance/                 # Engineering Foundation & Implementation Plans
├── apps/                           # Desktop Client Applications (Tauri + React)
├── packages/                       # Shared Rust & TypeScript Engine Packages
├── plugins/                        # Open-Source Plugin Modules
└── tools/                          # CI/CD Scripts & Build Tooling
```

---

## Documentation & Governance

- [Project Constitution v1.2.0](file:///D:/SiragugalFilmStudio/CONSTITUTION.md)
- [Immutable Architecture Principles](file:///D:/SiragugalFilmStudio/docs/architecture/ARCHITECTURE_PRINCIPLES.md)
- [Technology Decision Record (TDR)](file:///D:/SiragugalFilmStudio/docs/architecture/TECHNOLOGY_DECISION_RECORD.md)
- [Engineering Foundation & Standards](file:///D:/SiragugalFilmStudio/docs/governance/ENGINEERING_FOUNDATION.md)
- [Team Roles & Governance](file:///D:/SiragugalFilmStudio/docs/governance/TEAM_ROLES.md)

---

## Contributing & License

We welcome open-source contributions! Please read [CONTRIBUTING.md](file:///D:/SiragugalFilmStudio/CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](file:///D:/SiragugalFilmStudio/CODE_OF_CONDUCT.md) before submitting pull requests.

Licensed under the dual [Apache 2.0](file:///D:/SiragugalFilmStudio/LICENSE-APACHE) and [MIT License](file:///D:/SiragugalFilmStudio/LICENSE-MIT).
