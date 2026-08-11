# Native Project Format & Asset Database Architecture
**Siragugal Film Studio**  
**Document Version**: 1.1.0  
**Status**: APPROVED  
**Author**: AG (Chief Software Architect)  

---

## 1. Native Project Package Format (`.sfsp`)

Siragugal Film Studio uses a zero-copy, SQLite-backed container package format (`.sfsp` - Siragugal Film Studio Project).

### `.sfsp` Package Layout Structure
```
MyFilmProject.sfsp/
├── manifest.json              # Format version, project metadata, compatibility specs
├── project.db                 # Embedded SQLite database (Scenes, Timeline, Assets, Undo)
├── graph/
│   └── workflow.json          # Workflow Graph Engine DAG configuration
├── assets/
│   ├── video/                 # Raw & generated video clips
│   ├── audio/                 # Dialogue, score, foley audio files
│   └── image/                 # Storyboards, character stills, depth maps
└── models/
    └── fine_tunes/            # Project-specific character LoRA weights
```

---

## 2. Internal Asset Database Schema

The `project.db` relational database maintains track of all creative assets, relationships, and metadata:

```
[Characters] ──< [Actors] ──< [Voice Tracks]
     │
     ├──< [Costumes]
     │
     └──< [Scene Placements] ──> [Scenes] ──> [Timeline Tracks]
                                   │
                                   └──> [Generated Media]
```

### Managed Asset Entities:
- **Characters & Actors**: LoRA anchors, facial features, voice embeddings, character sheets.
- **Locations & Props**: 3D spatial maps, background plates, lighting presets.
- **Media & Generated Assets**: Video clips, audio tracks, storyboards, render passes.
- **Metadata**: AI prompts, seeds, model versions, generation timecodes, tags.

---

## 3. Universal Undo Architecture

Every creative interaction—including AI generation, script editing, camera adjustments, and timeline edits—is recorded in a persistent, transaction-based Undo Stack stored inside `project.db`:

```json
{
  "undo_id": "u-98421",
  "timestamp": "2026-08-03T09:20:00Z",
  "action_type": "AI_VIDEO_GENERATION",
  "studio_module": "Scene Studio",
  "target_node": "scene_4_shot_2",
  "previous_state": { "clip_hash": "a1b2...", "prompt": "wide shot forest" },
  "new_state": { "clip_hash": "c3d4...", "prompt": "wide shot forest sunset" },
  "reversible": true
}
```

- **Persistence**: Reversible even after closing and reopening the application.
- **Branching History**: Non-destructive tree-based undo allowing creators to branch alternative creative cuts.
