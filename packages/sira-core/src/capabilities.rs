/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AICapability {
    TextGeneration,
    ImageGeneration,
    VideoGeneration,
    AudioGeneration,
    SpeechToText,
    TextToSpeech,
    DepthEstimation,
    Upscaling,
}
