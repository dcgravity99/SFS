# MODULE 07 DESIGN SPECIFICATION: PROJECT (.SFSP) ENGINE
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 07 establishes the native project file package engine (`sfsp-engine`) for **Siragugal Film Studio**. It implements the zero-copy, SQLite-backed container package format (`.sfsp`), independent schema versioning, logical asset identifier resolution, atomic save and recovery procedures, optional SHA-256 integrity verification, safe project locking (`project.lock`), database migration framework, and reserved extension namespaces without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Formal Package Specification**: Implements package layout specified in [docs/architecture/SFSP_SPECIFICATION.md](file:///D:/SiragugalFilmStudio/docs/architecture/SFSP_SPECIFICATION.md).
2. **Independent Schema Versioning**: Parses `manifest.json` schema version headers (`manifest`, `database`, `asset_index`, `workflow_graph`). Guarantees 100% backward compatibility within 1.x series and emits structured error `SIRA-4002` for incompatible versions.
3. **Logical Asset Identifier Resolution**: Resolves assets using strongly typed UUID v7 `AssetId` handles for both internal (`assets/`) and external (`external://`) files.
4. **Atomic Save & Lock Engine**: Manages `project.lock` file with stale PID recovery, executing atomic staged saves (`.sfsp.tmp` -> sync -> atomic rename).
5. **SHA-256 Integrity Verification**: Verifies checksums of `manifest.json`, `project.db`, and `workflow.json` on project open.
6. **SQLite Database Migration Framework**: Executes versioned SQL migrations (`V1__init.sql`, `V2__add_index.sql`) automatically.
7. **Reserved Extension Namespaces**: Initializes sub-directories for `plugins/`, `cache/`, `previews/`, `ai/`, `exports/`, and `metadata/`.

---

## 3. File Blueprint

Module 07 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── sfsp-engine/                # Rust native project format package
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & SfspProject API
            ├── manifest.rs         # manifest.json parser, schema versioning & checksums
            ├── sqlite_db.rs        # project.db SQLite WAL mode wrapper & migration runner
            ├── lock.rs             # project.lock file manager & stale lock recovery
            ├── integrity.rs        # SHA-256 package integrity verification
            ├── archive.rs          # Compression & archive bundle packager
            └── migration.rs        # Format version migration framework
```

---

## 4. Acceptance Criteria

Module 07 is accepted when:
1. `packages/sfsp-engine` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. `.sfsp` package creation initializes all reserved sub-directories specified in `SFSP_SPECIFICATION.md`.
3. Logical `AssetId` UUID v7 resolution passes unit tests for internal and external asset URLs.
4. Backward compatibility within 1.x format series is verified, and unsupported versions trigger `SIRA-4002`.
5. Zero application or creative feature code is present.
