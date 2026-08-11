# ADR 0005: AI Capability Registry & SIRA Memory System Architecture
**Status**: APPROVED  
**Date**: 2026-08-03  
**Author**: AG (Chief Software Architect)  

## Context & Purpose
Hardcoding specific model names (e.g. `gpt-4o` or `sdxl-base`) inside application features creates vendor lock-in and breaks offline functionality when models are updated or unavailable.

## Quantifiable Benefits
- Decouples UI features from underlying AI models via an abstract Capability Registry.
- Allows automatic fallback from local high-VRAM models to quantised models or cloud APIs.
- SIRA Memory System maintains character and visual consistency across multi-scene film generations.

## Decision
All AI tasks must request capabilities (e.g. `Capability.VIDEO_GENERATION`) rather than specific model IDs. The Capability Registry maps requests to the user's preferred local model, fallback model, or cloud API.
