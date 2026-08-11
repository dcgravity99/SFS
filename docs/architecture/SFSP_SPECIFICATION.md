# NATIVE PROJECT PACKAGE FORMAT SPECIFICATION (.SFSP)
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: APPROVED & FROZEN  
**Author**: AG (Chief Software Architect)  

---

## 1. Executive Overview

The `.sfsp` (Siragugal Film Studio Project) package format is a zero-copy, SQLite-backed container directory and compressed archive format designed for 10+ year long-term stability, cross-platform portability, atomic transaction safety, and non-destructive undo persistence.

---

## 2. Directory Layout & Reserved Namespaces

```
MyFilmProject.sfsp/
├── manifest.json              # Format version, schema versions, project metadata & checksums
├── project.lock               # Process lockfile preventing concurrent write access
├── project.db                 # Embedded SQLite 3 database (WAL mode)
├── graph/
│   └── workflow.json          # Workflow Graph Engine DAG configuration
├── assets/                    # Managed internal media files
│   ├── video/                 # Video clips & render renders
│   ├── audio/                 # Dialogue, music, foley audio
│   └── image/                 # Storyboards, character stills, depth maps
├── models/
│   └── fine_tunes/            # Project-specific character LoRA weights
├── plugins/                   # Reserved: Project-specific plugin states
├── cache/                     # Reserved: Project preview & intermediate caches
├── previews/                  # Reserved: Low-res proxy & timeline thumbnails
├── ai/                        # Reserved: SIRA AI Memory & RAG vector indices
├── exports/                   # Reserved: Staged export renders (ProRes, H.264)
└── metadata/                  # Reserved: Custom studio tags & user annotations
```

---

## 3. Manifest Schema Specification (`manifest.json`)

```json
{
  "format_version": "1.0.0",
  "schema_versions": {
    "manifest": 1,
    "database": 1,
    "asset_index": 1,
    "workflow_graph": 1
  },
  "project_id": "018d9b12-42a1-7910-8b14-c12e5fa90123",
  "title": "Wings of Destiny",
  "created_at": "2026-08-03T10:10:00.000Z",
  "updated_at": "2026-08-03T10:10:00.000Z",
  "app_version_created": "v0.1.0-alpha",
  "app_version_min": "v0.1.0-alpha",
  "integrity": {
    "db_sha256": "31e35cde5e003f1d0541e21...89a",
    "workflow_sha256": "4bf92f3577b34da6a3ce9...736"
  }
}
```

---

## 4. Asset References & Logical Identifiers

All asset references in `project.db` MUST use strongly typed logical UUID v7 identifiers (`AssetId`) instead of absolute file paths:

- **Managed Internal Asset**: `assets/video/018d9b12-42a1-7910-8b14-c12e5fa90123.mp4`
- **External Referenced Asset**: `external://D:/RawFootage/Reel1/Shot04.mov` (with fallback local proxy in `previews/`).

---

## 5. Atomic Save & Transaction Recovery

1. **Atomic Write Pipeline**: Changes are staged in `MyFilmProject.sfsp.tmp/`. Upon successful commit, `manifest.json` and `project.db` are synced, and the directory swap occurs atomically.
2. **Locking & Recovery**: `project.lock` records PID and hostname. Stale locks (>10 minutes inactive PID) trigger automated safe recovery.
