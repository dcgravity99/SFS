# Siragugal Film Studio

# Module 61 — AI Director Decision Engine

## Implementation Plan

Status:
Planning

Architect:
AG (Gemini 3.6 Flash High)

---

# 1. Implementation Location

Package:

packages/sira-engine-director

Module:

ai_director_decision_engine

Language:

Rust

Runtime:

Tokio async service

---

# 2. Architecture Boundary

The engine operates as:

Story Engine
        |
        |
        v
AI Director Decision Engine
        |
        |
        v
DirectorDecisionGraph


The engine does not:

- render frames
- modify assets
- call external AI providers
- override user decisions

---

# 3. Core Data Structures

Planned:

DirectorRequest

Fields:

- project_id
- scene_id
- story_context
- character_context
- available_assets
- timeline_context


DirectorDecision

Fields:

- decision_id
- recommendation_type
- explanation
- confidence
- approval_required
- decision_timestamp
- engine_version
- reasoning_trace_id


---

# 4. Service Interface

Communication:

gRPC

Service:

DirectorDecisionService


Operations:

- AnalyzeScene
- RecommendShots
- EvaluateEmotion
- DetectContinuityIssues
- GenerateDirectorNotes


---

# 5. Dependencies

Internal:

- Story Engine
- Scene Engine
- Timeline Engine
- Asset Engine
- SIRA Core Runtime


External:

None required


---

# 6. Runtime Integration

Process model:

SIRA Core Runtime
        |
        |
        +-- Director Decision Service


Requirements:

- isolated process
- crash supervision
- structured logging
- deterministic execution


---

# 7. Testing Implementation Mapping

Unit:

- scene analysis
- shot recommendation
- confidence scoring


Integration:

- Story → Director
- Scene → Director
- Timeline → Director


Safety:

- approval enforcement
- audit validation
- deterministic output


---

# 8. Implementation Constraints

Must follow:

- Human in Control
- Offline First
- Provider Agnostic
- Explainable AI
- Deterministic Outputs

No UI implementation.

No rendering implementation.

No external AI dependency.

