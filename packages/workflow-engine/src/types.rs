/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalDataType {
    Story,
    SceneList,
    CharacterList,
    Dialogue,
    VoiceTrack,
    VideoClip,
    Image,
    Subtitle,
    Timeline,
    Prompt,
    Embedding,
    Metadata,
    RawBuffer,
}
