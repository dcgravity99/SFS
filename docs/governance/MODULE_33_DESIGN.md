# MODULE 33 DESIGN SPECIFICATION: CHARACTER STUDIO UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 33 establishes the **Character Studio UI** (`apps/studio-ui/src/features/character/`) for **Siragugal Film Studio**. It implements visual character profile creation, facial identity anchor galleries, costume variation references, project-level LoRA weight binding panels (`.safetensors`), visual consistency distance meters, and live integration with `sira_engine_character` (Module 18) without adding unapproved rendering or AI video generation features.

---

## 2. Module Responsibilities & Core Features

1. **Character Visual Identity Profile**: Interactive card gallery displaying character visual anchors (Facial embeddings, hairstyle, costume variations).
2. **Project-Level LoRA Model Binding Panel**: Bind `.safetensors` LoRA weights to `CharacterId` handles stored in `asset_db` with SHA-256 integrity verification.
3. **Visual Consistency Meter**: Calculate visual embedding similarity metrics between anchor references and target shot embeddings (`0.0` to `1.0`).
4. **Sub-Engine IPC Integration**: Send character profile actions to `sira_engine_character` via `StudioIpcService.executeEngineCommand('character_create', ...)` and `character_bind_lora`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 18 (`sira_engine_character`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 32 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_32_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 33 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/character/types.ts)
export interface CharacterProfileView {
  character_id: string;
  name: string;
  role: string;
  voice_model_id?: string;
  lora_weight_path?: string;
  visual_anchor_count: number;
  consistency_score: number; // 0.0 to 1.0
}

// React Feature Components
export declare const CharacterGallery: React.FC;
export declare const LoraBindingPanel: React.FC;
export declare const VisualConsistencyMeter: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 33 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            └── features/
                └── character/      # Character Studio feature package
                    ├── types.ts    # Character UI data models
                    ├── CharacterGallery.tsx       # Character visual profile gallery
                    ├── LoraBindingPanel.tsx       # LoRA model binding panel
                    └── VisualConsistencyMeter.tsx # Visual similarity meter
```

---

## 6. Testing & Validation Strategy

1. **Character Profile Creation Test**: Create character profile; verify `CharacterId` UUID v7 is generated and rendered in gallery.
2. **LoRA Binding Integration Test**: Bind `.safetensors` model path; verify IPC command sends path to `sira_engine_character`.
3. **Visual Consistency Score Meter Test**: Supply `0.92` similarity score; verify score meter displays green status indicator.

---

## 7. Acceptance Criteria

Module 33 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Character gallery renders profiles and binds LoRA weights via IPC cleanly.
3. Visual consistency meter displays similarity scores with WCAG 2.2 AA accessibility support.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 33: Character Studio UI**.
> 2. Upon your explicit approval, I will execute Module 33 implementation (`apps/studio-ui/src/features/character/`).
