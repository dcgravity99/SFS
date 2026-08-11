# MODULE 40 DESIGN SPECIFICATION: AI PROMPT BUILDER & GENERATION WORKSPACE UI
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 40 establishes the **AI Prompt Builder & Generation Workspace UI** (`apps/studio-ui/src/features/prompts/`) for **Siragugal Film Studio**. It implements structured AI prompt construction panels (Positive Prompt, Negative Prompt, Character LoRA bindings, Style presets), generation parameter controls (CFG Scale, Seed, Steps, Sampler), model profile selectors (`sira_ai_provider` handles), and live IPC integration with `sira_ai_provider` (Module 09) following the Tamil-first (`ta-IN`) globalization architecture without executing unapproved AI inference locally inside React UI.

---

## 2. Module Responsibilities & Core Features

1. **Structured AI Prompt Builder**: Prompt construction panel supporting positive prompts, negative prompts, shot intent tags, and style template presets.
2. **Character & Style LoRA Weight Selector**: Panel binding `CharacterId` visual anchors and style LoRA weights (`AssetId` handles) to prompt generation jobs.
3. **Generation Parameter Inspector**: Numerical controls for CFG Scale (`1.0` to `20.0`), Sampling Steps (`10` to `150`), Random Seed (`-1` for random), and Sampler Selection (`Euler A`, `DPM++ 2M`).
4. **Model Provider Profile Selector**: Model dropdown selecting registered AI provider profiles (`ModelId` handles) managed by `sira_ai_provider` (Module 09).
5. **Globalization & Localization Engine**: Tamil-first i18n string externalization (`ta-IN` primary, `en-US` secondary) for all prompt builder controls.
6. **Sub-Engine IPC Integration**: Send prompt generation jobs to `sira_ai_provider` via `StudioIpcService.executeEngineCommand('ai_submit_generation_job', ...)` and `ai_cancel_job`.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 09 (`sira_ai_provider`), Module 08 (`sira_core`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 39 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_39_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 40 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/prompts/types.ts)
export interface AiPromptSpecView {
  prompt_id: string; // Machine-readable UUIDv7
  positive_prompt: string;
  negative_prompt: string;
  model_id: string; // ModelId reference
  lora_asset_ids: string[]; // AssetId references only
  cfg_scale: number;
  steps: number;
  seed: number;
  sampler_name: string;
}

export interface PromptTemplatePresetView {
  preset_id: string;
  name: LocalizedTextMap;
  description: LocalizedTextMap;
}

// React Feature Components
export declare const PromptBuilderWorkspace: React.FC;
export declare const PositivePromptEditor: React.FC;
export declare const NegativePromptEditor: React.FC;
export declare const GenerationParamsPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 40 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            ├── i18n/
            │   └── locales/
            │       ├── ta-IN/
            │       │   └── prompts.json
            │       └── en-US/
            │           └── prompts.json
            └── features/
                └── prompts/        # AI Prompt Builder feature package
                    ├── types.ts    # Prompt & generation UI models
                    ├── PromptBuilderWorkspace.tsx # Master prompt builder workspace
                    ├── PositivePromptEditor.tsx   # Positive prompt input & tag selector
                    ├── NegativePromptEditor.tsx   # Negative prompt editor
                    └── GenerationParamsPanel.tsx  # CFG, Steps, Seed & Sampler inspector
```

---

## 6. Testing & Validation Strategy

1. **Prompt Spec Assembly Test**: Enter positive prompt; verify payload assembles valid `AiPromptSpecView` object.
2. **Generation Job Submit IPC Test**: Click Submit Job; verify IPC payload emits `ai_submit_generation_job` command cleanly.
3. **Tamil Localization Compliance Test**: Switch to `ta-IN`; verify headers render in Tamil (`செயற்கை நுண்ணறிவு குறிப்புத் தயாரிப்பு`).

---

## 7. Acceptance Criteria

Module 40 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. AI Prompt Builder components render prompt editors, negative prompt panels, and parameter controls cleanly.
3. Tamil-first localization (`ta-IN`) functions cleanly across all prompt builder controls.
4. Zero unapproved local AI model inference code is present in React UI.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 40: AI Prompt Builder & Generation Workspace UI**.
> 2. Upon your explicit approval, I will execute Module 40 implementation (`apps/studio-ui/src/features/prompts/`).
