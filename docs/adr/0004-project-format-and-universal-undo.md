# ADR 0004: Native Project Package Format (.sfsp) & Universal Undo Stack
**Status**: APPROVED  
**Date**: 2026-08-03  
**Author**: AG (Chief Software Architect)  

## Context & Purpose
The studio requires a unified, portable, non-destructive project file format (`.sfsp`) capable of storing timeline state, character assets, workflow graphs, and persistent universal undo history.

## Quantifiable Benefits
- Zero-copy SQLite embedded database (`project.db`) inside package directory allows instant project loading.
- Reversible Universal Undo stack remains persistent across application restarts.
- Structured schema supports automated version migration.

## Identified Risks & Mitigation
- **Risk**: Growing database size due to undo history accumulation.
- **Mitigation**: Configurable undo history pruning (e.g. keep last 500 actions or compress state snapshots).

## Migration Strategy & Backward Compatibility
- `.sfsp` contains a mandatory `manifest.json` schema version key. Future versions will execute automated migration scripts on project open.
