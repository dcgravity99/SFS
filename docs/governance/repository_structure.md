# Repository Architecture & Directory Blueprint
**Siragugal Film Studio**  
**Document Version**: 1.2.0  
**Status**: APPROVED & FROZEN  
**Author**: AG (Chief Software Architect)  

---

## Workspace Directory Layout (v1.2.0)

```
D:\SiragugalFilmStudio\
├── CONSTITUTION.md                 # Project Constitution v1.2.0
├── README.md                       # Open Source Overview & Contributor Guide
├── docs/                           # Central Engineering Documentation
│   ├── README.md
│   ├── architecture/               # System & Sub-engine Specifications
│   │   ├── ARCHITECTURE_PRINCIPLES.md # Immutable Engineering Principles
│   │   ├── TECHNOLOGY_DECISION_RECORD.md # Tech Stack Choices & Analysis
│   │   ├── ARCHITECTURE_READINESS_REVIEW.md # Readiness Audit & Risk Register
│   │   ├── system_architecture.md  # Master 4-Tier System Architecture
│   │   ├── ai_architecture.md      # SIRA AI Core, Model Registry & Workflows
│   │   ├── hardware_architecture.md# Hardware Abstraction Layer (HAL)
│   │   ├── project_format_and_asset_db.md # Native .sfsp Format & Asset DB
│   │   ├── render_architecture.md  # Enterprise Render Scheduler
│   │   ├── plugin_architecture.md  # Sandboxed Plugin SDK (10 Categories)
│   │   ├── experience_layer.md     # Experience Layer Architecture
│   │   ├── templates_and_marketplace.md # AI Workflow Marketplace & Templates
│   │   └── resource_and_cache_architecture.md # Resource Manager & Caching
│   ├── adr/                        # Architecture Decision Records
│   │   ├── 0001-record-architecture-decisions.md
│   │   ├── 0002-sira-ai-core-orchestration.md
│   │   ├── 0003-hardware-abstraction-layer.md
│   │   ├── 0004-project-format-and-universal-undo.md
│   │   ├── 0005-ai-capability-registry-and-memory-system.md
│   │   └── 0006-resource-manager-and-cache-engine.md
│   ├── governance/                 # Development, Security & Repository Standards
│   │   ├── repository_structure.md
│   │   └── development_workflow.md
│   ├── prompts/                    # Master Requirement Briefs & Design Specs
│   │   ├── README.md
│   │   └── studio_brief.md
│   └── roadmap/                    # Multi-Year Phased Development Plan
│       └── phased_roadmap.md
├── apps/                           # Desktop Client Applications
│   └── desktop/                    # Tauri / Rust Native Shell + React (14 Creative Studio Modules)
├── packages/                       # Core Shared Engines & Libraries
│   ├── sira-core/                  # SIRA AI Core Runtime & 11 Sub-Engines
│   ├── hal/                        # Hardware Abstraction Layer (C++/Rust Bindings)
│   ├── media-pipeline/             # Native C++/FFmpeg/GPU Video Processing Engine
│   └── plugin-sdk/                 # Extension SDK for Third-Party Developers
├── plugins/                        # Built-in Open-Source Extensions
│   ├── ai-providers/               # Local (Ollama, ComfyUI) & Cloud (OpenAI, Gemini)
│   ├── input-parsers/              # Voice, Text, Script, PDF, Web Scraping Plugins
│   └── media-enhancers/            # Upscalers, Audio Denoise, Frame Interpolation
└── tools/                          # CI/CD Scripts, Test Harnesses & Build Tooling
```
