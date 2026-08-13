# Module 61 Interfaces

## Input Interface

DirectorRequest

Fields:

- project_id
- scene_id
- story_context
- character_context
- available_assets


## Output Interface

DirectorDecision

Fields:

- decision_id
- recommendation_type
- explanation
- confidence
- approval_required


## Communication

Recommended:

gRPC

Runtime:

Rust Tokio service

Transport:

Existing SIRA Core Runtime architecture
