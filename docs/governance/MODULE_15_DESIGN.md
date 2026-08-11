# MODULE 15 DESIGN SPECIFICATION: CACHE MANAGER
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED DESIGN SPECIFICATION  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 15 establishes the multi-tiered Cache Manager (`cache-manager`) for **Siragugal Film Studio**. It implements 8 cache categories, SQLite metadata indexing (`cache.db`), hybrid smart eviction policies, background maintenance services, deep Resource Manager memory spilling, AI Model Residency management, crash recovery, and distributed cache extension points without adding application-level feature logic.

---

## 2. Module Responsibilities & Core Features

1. **8 Formally Defined Cache Categories**:
   - `ModelCache`: Model weights (GGUF, Safetensors).
   - `TensorCache`: Intermediate neural activation tensors.
   - `EmbeddingCache`: Text & image vector embeddings.
   - `VideoFrameCache`: Uncompressed & compressed video frames.
   - `AudioWaveformCache`: Processed audio PCM buffers.
   - `ThumbnailCache`: UI storyboards & timeline thumbnails.
   - `ProxyMediaCache`: Low-resolution proxy media clips.
   - `WorkflowIntermediateCache`: Node output artifacts.
2. **SQLite Cache Metadata Index (`cache.db`)**: Stores metadata: `cache_key`, `category`, `sha256`, `size_bytes`, `created_at`, `last_accessed_at`, `access_count`, `owner_module`, `workflow_id`, `expiration_policy`, `compression_method`.
3. **Smart Hybrid Eviction Engine**: Supports `LRU`, `LFU`, `TTL`, `PriorityBased`, `CostBased`, and `Hybrid` eviction policies.
4. **Background Cache Maintenance Service**: Runs orphan cleanup, expired entry purging, checksum integrity verifications, and SSD trimming out-of-band.
5. **Resource Manager Memory Spilling**: Communicates with `resource_manager` to spill RAM cache to SSD when memory pressure rises, preventing OOM panics.
6. **AI Model Residency Management**: Coordinates with `sira_ai_provider` to pin hot models in VRAM/RAM while unloading idle weights.
7. **Crash Recovery & Integrity Repair**: Repairs corrupted `cache.db` indexes and removes orphaned partial files on application startup.
8. **Distributed Cache Extension Point**: Exposes traits for future LAN render farm and cloud network caches.

---

## 3. SQLite Cache Index Schema (`cache.db`)

```sql
CREATE TABLE IF NOT EXISTS cache_index (
    cache_key TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    sha256 TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_accessed_at TEXT NOT NULL,
    access_count INTEGER DEFAULT 1,
    owner_module TEXT NOT NULL,
    workflow_id TEXT,
    expiration_policy TEXT DEFAULT 'LRU',
    compression_method TEXT DEFAULT 'NONE'
);
CREATE INDEX IF NOT EXISTS idx_cache_category ON cache_index(category);
CREATE INDEX IF NOT EXISTS idx_cache_accessed ON cache_index(last_accessed_at);
```

---

## 4. File Blueprint

Module 15 implements the following crate structure:

```
D:\SiragugalFilmStudio\
└── packages/
    └── cache-manager/              # Rust Cache Manager crate
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Export root & CacheManager API
            ├── categories.rs       # 8 CacheCategory enums & independent quota definitions
            ├── index_db.rs         # SQLite cache.db metadata indexer
            ├── eviction.rs         # Smart hybrid eviction engine (LRU, LFU, Cost-Based)
            ├── maintenance.rs      # Background maintenance & orphan cleanup service
            ├── residency.rs        # AI Model Residency Manager (RAM/VRAM pinning)
            ├── keys.rs             # SHA-256 cache key generator
            ├── ram_cache.rs        # Tier 1 RAM cache with Resource Manager spilling
            ├── disk_cache.rs       # Tier 2 NVMe SSD disk cache
            ├── telemetry.rs        # Expanded cache hit/miss & RAM/VRAM savings sampler
            └── recovery.rs         # Startup crash recovery & checksum repair engine
```

---

## 5. Acceptance Criteria

Module 15 is accepted when:
1. `packages/cache-manager` builds cleanly with zero compiler warnings (`#[deny(warnings)]`).
2. 8 cache categories operate with independent quotas and SQLite `cache.db` metadata indexing.
3. Memory pressure events trigger automatic RAM-to-SSD cache spilling without data loss.
4. Startup recovery repairs orphan files and corrupted indexes cleanly.
5. Zero application or creative feature code is present.
