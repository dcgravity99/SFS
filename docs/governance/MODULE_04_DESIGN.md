# MODULE 04 DESIGN SPECIFICATION: CONFIGURATION SYSTEM
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 04 establishes the enterprise configuration loading engine (`sira-config`) for **Siragugal Film Studio**. It implements the strict 6-tier configuration resolution hierarchy, configuration schema versioning, automated migration with backups, secret separation via OS keychains, static vs dynamic setting classifications, multi-stage validation, and configuration diagnostics without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **6-Tier Resolution Hierarchy**:
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
2. **Configuration Schema Versioning & Auto-Migration**: Every configuration file includes an explicit `version` field. Legacy versions trigger automated migration functions with timestamped backup creation (`studio.json.v1.bak`).
3. **Secret Separation**: Sensitive data (API keys, OAuth tokens) are strictly excluded from JSON config files and stored in platform-native secure keychains (macOS Keychain / Windows Credential Manager).
4. **Static vs Dynamic Settings Classification**: Settings are tagged as `Static` (requires process restart, e.g. IPC socket paths) or `Dynamic` (hot-reloadable at runtime, e.g. render thread priority).
5. **Configuration Observer Pattern**: Provides a thread-safe change notification bus (`ConfigObserver`) emitting setting update events to subscribed modules when dynamic settings change.
6. **Multi-Stage Validation Pipeline**:
   - *Stage 1: Syntax Validation* (Valid JSON/TOML parser check).
   - *Stage 2: Schema Validation* (Types, required keys, value bounds).
   - *Stage 3: Semantic Validation* (Cross-setting invariants e.g. VRAM limit < system RAM).
   - *Stage 4: Platform Validation* (Verifies path existence & hardware backend availability).
7. **Configuration Diagnostics (`sira-config explain`)**: Tracks the exact originating layer (Defaults, System, User, Project, Env, CLI) for every effective configuration value.
8. **Plugin Config Schema Registration**: Sandboxed plugins register custom configuration schemas safely under isolated namespaces (`plugins.<plugin_id>.<key>`).

---

## 3. Configuration Diagnostics & Traceability Example

```json
{
  "effective_config": {
    "hal.vram_limit_mb": {
      "value": 16384,
      "origin_layer": "ENVIRONMENT_VARIABLE",
      "source_detail": "SIRA_HAL_VRAM_LIMIT_MB=16384"
    },
    "render.max_threads": {
      "value": 8,
      "origin_layer": "USER_CONFIG",
      "source_detail": "~/.config/sira/studio.json"
    },
    "logging.level": {
      "value": "INFO",
      "origin_layer": "BUILTIN_DEFAULT",
      "source_detail": "sira_config::schema::defaults"
    }
  }
}
```

---

## 4. File Blueprint

Module 04 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sira-config/                # Rust configuration engine crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & ConfigManager singleton
            ├── schema.rs           # SiraConfig struct, static/dynamic tags, defaults
            ├── hierarchy.rs        # 6-tier configuration resolution engine
            ├── env_map.rs          # SIRA_* environment variable mapper
            ├── migration.rs        # Schema versioning & backup migration framework
            ├── diagnostics.rs      # Provenance tracking & sira-config explain engine
            └── observer.rs         # Dynamic setting change notification bus
```

---

## 5. Acceptance Criteria

Module 04 is accepted when:
1. `packages/sira-config` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. The 6-tier hierarchy resolution tests pass 100%.
3. Multi-stage validation catches syntax, schema, semantic, and platform errors cleanly, emitting structured error `SIRA-1002`.
4. Configuration diagnostics trace 100% of effective configuration values to their originating source layers.
5. Zero application or creative feature code is present.
