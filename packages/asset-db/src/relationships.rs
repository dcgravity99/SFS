/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipOntology {
    Contains,
    DependsOn,
    GeneratedFrom,
    DerivedFrom,
    UsesVoice,
    PlacedInScene,
    WornBy,
}

impl RelationshipOntology {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Contains => "CONTAINS",
            Self::DependsOn => "DEPENDS_ON",
            Self::GeneratedFrom => "GENERATED_FROM",
            Self::DerivedFrom => "DERIVED_FROM",
            Self::UsesVoice => "USES_VOICE",
            Self::PlacedInScene => "PLACED_IN_SCENE",
            Self::WornBy => "WORN_BY",
        }
    }
}
