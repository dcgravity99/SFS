# MODULE 41 DESIGN SPECIFICATION: ASSET & MEDIA MANAGEMENT UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 41 establishes the **Asset & Media Management UI** (`apps/studio-ui/src/features/assets/`) for **Siragugal Film Studio**. It implements centralized digital asset management (DAM) interfaces, media asset galleries (Video, Audio, 3D Mesh, Texture, LoRA Model assets), asset metadata inspectors (`AssetId` handles, MIME types, file sizes, SHA-256 checksums), storage quota visualizers, and live IPC integration with `sira_asset_db` (Module 05) following the Tamil-first (`ta-IN`) globalization architecture while strictly prohibiting direct filesystem access from React UI.

---

## 2. Module Responsibilities & Core Features

1. **Asset Management Workspace**: Master digital asset management workspace coordinating project media assets.
2. **Media Asset Gallery**: Categorized grid/list gallery displaying asset thumbnails, asset categories (`Video`, `Audio`, `Model`, `Texture`), and `AssetId` handles.
3. **Asset Metadata Inspector**: Detailed inspector displaying `AssetId`, MIME type, resolution/sample rate, byte size, creation timestamp, and SHA-256 cryptographic hash.
4. **Storage Quota & VRAM Allocator Panel**: Gauge panel visualizing local storage quota utilization and cached model asset memory usage.
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all asset management controls.
6. **Sub-Engine IPC Integration**: Send asset catalog queries to `sira_asset_db` via `StudioIpcService.executeEngineCommand('asset_register', ...)` and `asset_query_by_id`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 05 (`sira_asset_db`), Module 08 (`sira_core`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 40 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_40_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 41 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/assets/types.ts)
export interface AssetMetadataView {
  asset_id: string; // Machine-readable UUIDv7 handle
  display_name: LocalizedTextMap;
  asset_category: 'Video' | 'Audio' | 'Model' | 'Texture' | 'Script';
  mime_type: string;
  file_size_bytes: number;
  sha256_checksum: string;
  created_at: string;
}

export interface StorageQuotaView {
  total_quota_bytes: number;
  used_bytes: number;
  cached_models_bytes: number;
}

// React Feature Components
export declare const AssetWorkspace: React.FC;
export declare const MediaAssetGallery: React.FC;
export declare const AssetMetadataInspector: React.FC;
export declare const StorageQuotaPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 41 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── assets.json
            │       └── en-US/
            │           └── assets.json
            └── features/
                └── assets/         # Asset & Media Management feature package
                    ├── types.ts    # Asset metadata & storage UI models
                    ├── AssetWorkspace.tsx        # Master asset workspace
                    ├── MediaAssetGallery.tsx      # Categorized asset gallery grid
                    ├── AssetMetadataInspector.tsx # AssetId & SHA-256 metadata panel
                    └── StorageQuotaPanel.tsx     # Storage quota & cache inspector
```

---

## 6. Testing & Validation Strategy

1. **Asset Gallery Categorization Test**: Filter by `Audio`; verify only audio assets are displayed in gallery.
2. **SHA-256 Checksum Inspection Test**: Select asset; verify SHA-256 checksum string displays cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`வளக் கூடம் (Asset Gallery)`).

---

## 7. Acceptance Criteria

Module 41 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Asset Management components render media galleries, metadata inspectors, and quota panels cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all asset controls.
4. Zero direct React filesystem access or absolute path storage is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 41: Asset & Media Management UI**.
> 2. Upon your explicit approval, I will execute Module 41 implementation (`apps/studio-ui/src/features/assets/`).
