# ADR 0006: Centralized Resource Manager & Media Cache Engine Architecture
**Status**: APPROVED  
**Date**: 2026-08-03  
**Author**: AG (Chief Software Architect)  

## Context & Purpose
High-resolution 4K/8K video processing and AI diffusion model inference can exhaust system RAM, VRAM, and disk space if unmanaged, leading to system freezes and crash events.

## Quantifiable Benefits
- Centralized Resource Manager enforces thermal, battery, and VRAM quotas across all background workers.
- Multi-tier Media Cache Engine (proxy, thumbnail, intermediate tensor, waveform) delivers 60fps timeline scrubbing performance.
- LRU cache eviction prevents disk exhaustion.

## Decision
The Resource Manager and Media Cache Engine are integrated into the core runtime, coordinating directly with the Enterprise Render Scheduler and SIRA Model Manager.
