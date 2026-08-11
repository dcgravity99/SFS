# MODULE 32 DESIGN SPECIFICATION: PROFESSIONAL SCREENPLAY WRITER & STORY STUDIO
**Siragugal Film Studio**  
**Document Version**: 1.0.0  
**Status**: PROPOSED FOR USER REVIEW & APPROVAL  
**Author**: AG (Chief Software Architect)  

---

## 1. Module Purpose

Module 32 establishes the **Professional Screenplay Writer & Story Studio UI** (`apps/studio-ui/src/features/story/`) for **Siragugal Film Studio**. It implements standard screenplay Fountain editor views (INT/EXT scene headings, character cues, dialogue blocks, parentheticals, action lines), 3-Act story beat sheet visualizers (`Opening Image`, `Catalyst`, `Midpoint`, `Climax`), character speech word count analyzers, and live integration with `sira_engine_story` (Module 17) without adding unapproved rendering or AI video features.

---

## 2. Module Responsibilities & Core Features

1. **Fountain Screenplay Editor**: Specialized script formatting editor supporting Fountain `.fountain` text syntax auto-formatting (Scene Headings, Action, Character, Dialogue, Parentheticals).
2. **Interactive 3-Act Beat Sheet Visualizer**: Visual beat sheet board displaying narrative act progression (`Opening Image`, `Catalyst`, `Break into Two`, `Midpoint`, `Bad Guys Close In`, `All Is Lost`, `Climax`, `Resolution`).
3. **Scene Breakdown & Character Dialogue Analyzer**: Interactive side panel displaying scene lists, character line counts, and dialogue word distribution graphs.
4. **Sub-Engine IPC Integration**: Send screenplay text to `sira_engine_story` via `StudioIpcService.executeEngineCommand('story_parse_fountain', ...)` and render structured `ScriptAST` items dynamically.

---

## 3. Module Dependencies

- **Software Dependencies**: Module 31 (`apps/studio-ui`), Module 17 (`sira_engine_story`), Module 01 (`sira_types`), React 19, Zustand stores (`useProjectStore`, `useWorkspaceStore`), Tailwind CSS, Lucide Icons.
- **Module Dependencies**: Depends on [Module 31 Completion](file:///D:/SiragugalFilmStudio/docs/governance/MODULE_31_COMPLETION.md).

---

## 4. Public Interfaces & Component Architecture

Module 32 exposes the following React components and TypeScript models:

```typescript
// TypeScript Component Props & Interfaces (src/features/story/types.ts)
export interface ScreenplaySceneView {
  scene_number: number;
  heading: string;
  action_lines: string[];
  dialogue_blocks: Array<{
    character_name: string;
    parenthetical?: string;
    speech_text: string;
  }>;
}

export interface StoryBeatView {
  beat_id: string;
  beat_type: string;
  scene_ids: number[];
  description: string;
}

// React Feature Components
export declare const ScreenplayEditor: React.FC;
export declare const BeatSheetBoard: React.FC;
export declare const SceneBreakdownPanel: React.FC;
```

---

## 5. Internal Structure & File Blueprint

Upon approval of this design document, Module 32 will create the following feature directory structure:

```
D:\SiragugalFilmStudio\
└── apps/
    └── studio-ui/
        └── src/
            └── features/
                └── story/          # Story Studio feature package
                    ├── types.ts    # Screenplay & beat sheet UI models
                    ├── ScreenplayEditor.tsx  # Fountain screenplay editor
                    ├── BeatSheetBoard.tsx    # 3-Act visual beat sheet
                    └── SceneBreakdownPanel.tsx# Character & scene breakdown
```

---

## 6. Testing & Validation Strategy

1. **Screenplay Auto-Formatting Unit Test**: Enter Fountain text (`INT. SOUNDSTAGE - DAY`); verify scene heading auto-formats correctly.
2. **Beat Sheet Rendering Integration Test**: Supply `StoryBeatView` array; verify 3-Act beat cards render in chronological order.
3. **Dialogue Word Count Calculation Test**: Calculate character dialogue word counts; verify numbers match expected count.

---

## 7. Acceptance Criteria

Module 32 is accepted when:
1. `apps/studio-ui` builds cleanly with zero TypeScript errors under strict mode.
2. Fountain screenplay editor formats script text and syncs with `sira_engine_story` via IPC cleanly.
3. 3-Act beat sheet visualizer renders story beats with WCAG 2.2 AA accessibility support.
4. Zero unapproved AI generation code is present.

---

## 8. Next Action

> [!IMPORTANT]
> Per the mandatory workflow rule:
> 1. Please review this design document for **Module 32: Professional Screenplay Writer & Story Studio**.
> 2. Upon your explicit approval, I will execute Module 32 implementation (`apps/studio-ui/src/features/story/`).
