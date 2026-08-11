# MODULE 08 DESIGN SPECIFICATION: ASSET DATABASE
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 08 establishes the embedded relational Asset Database engine (`asset-db`) for **Siragugal Film Studio**. It manages entity schemas, extensible asset type registries, universal metadata models, formal relationship ontologies, weighted FTS5 search indexing, complete asset lifecycle state machines, version graphs, performance database indexes, plugin-owned extension tables, and asset mutation event dispatchers stored inside `project.db` without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **Extensible `AssetTypeRegistry`**: Supports built-in asset types (`Video`, `Audio`, `Image`, `Character`, `Actor`, `Voice`, `Location`, `Prop`, `Style`, `Prompt`, `Storyboard`, `Script`) and plugin-registered custom types.
2. **Universal Asset Metadata Model**: Every asset contains standardized fields: `asset_id` (UUID v7), `asset_type`, `lifecycle_status`, `checksum_sha256`, `tags`, `metadata_json`, `created_at`, `updated_at`.
3. **Formal Relationship Ontology**: Strictly typed directed relationships (`CONTAINS`, `DEPENDS_ON`, `GENERATED_FROM`, `DERIVED_FROM`, `USES_VOICE`, `PLACED_IN_SCENE`, `WORN_BY`).
4. **Weighted FTS5 & Vector Hooks**: Full-text search with weighted field ranking (title 3x, tags 2x, description 1x), fuzzy search, and vector embedding hooks (`vec_embeddings` table).
5. **Asset Lifecycle State Machine**: Enforces valid transitions across 8 states (`Draft` → `Generated` → `Imported` → `Edited` → `Approved` → `Published` → `Archived` → `SoftDeleted`).
6. **Version Graph Architecture**: Replaces linear versions with a DAG version tree supporting branches and merges (`parent_asset_ids`).
7. **Plugin Extension Tables**: Allows plugins to create isolated extension tables linked via `asset_id` without altering core DDL schemas.
8. **Structured Asset Mutation Events**: Emits `AssetMutationEvent` payloads (Created, Updated, StatusChanged, RelationshipLinked, SoftDeleted) to observer buses.

---

## 3. Database Schema & Index Specifications (`project.db`)

```sql
-- Core Assets Table with Lifecycle & Metadata
CREATE TABLE assets (
    asset_id TEXT PRIMARY KEY,          -- UUID v7
    asset_type TEXT NOT NULL,
    lifecycle_status TEXT NOT NULL,     -- 'Draft', 'Generated', 'Approved', etc.
    display_name TEXT NOT NULL,
    uri TEXT NOT NULL,                  -- 'assets/video/...' or 'external://...'
    mime_type TEXT NOT NULL,
    file_size_bytes INTEGER NOT NULL,
    checksum_sha256 TEXT NOT NULL,
    version_branch TEXT DEFAULT 'main',
    parent_asset_ids TEXT,              -- JSON array of parent UUIDs for branching
    metadata_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Performance Indexes
CREATE INDEX idx_assets_type_status ON assets(asset_type, lifecycle_status);
CREATE INDEX idx_assets_checksum ON assets(checksum_sha256);

-- Formal Relationship Ontology Table
CREATE TABLE asset_relationships (
    relationship_id TEXT PRIMARY KEY,
    source_asset_id TEXT NOT NULL,
    target_asset_id TEXT NOT NULL,
    relationship_ontology TEXT NOT NULL, -- 'CONTAINS', 'DEPENDS_ON', 'GENERATED_FROM', etc.
    metadata_json TEXT,
    FOREIGN KEY(source_asset_id) REFERENCES assets(asset_id) ON DELETE CASCADE,
    FOREIGN KEY(target_asset_id) REFERENCES assets(asset_id) ON DELETE CASCADE
);

CREATE INDEX idx_rel_source_ontology ON asset_relationships(source_asset_id, relationship_ontology);
```

---

## 4. File Blueprint

Module 08 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── asset-db/                   # Rust embedded asset database crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & AssetDatabase API
            ├── schema.rs           # DDL migrations & performance index creation
            ├── type_registry.rs    # Extensible AssetTypeRegistry & plugin registration
            ├── records.rs          # Universal AssetRecord & RelationshipRecord structs
            ├── lifecycle.rs        # Lifecycle state machine & transition rules
            ├── query.rs            # Weighted FTS5 search & vector-search hooks
            ├── relationships.rs    # Formal relationship ontology manager
            └── events.rs           # AssetMutationEvent dispatcher
```

---

## 5. Acceptance Criteria

Module 08 is accepted when:
1. `packages/asset-db` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. Asset CRUD, 8-state lifecycle transitions, version branch DAGs, and relationship ontology operations pass 100% of unit tests.
3. FTS5 weighted ranking search queries execute cleanly against `project.db`.
4. Schema migrations preserve all metadata and relationships with post-migration integrity verification.
5. Zero application or creative feature code is present.
