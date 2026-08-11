/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 * Licensed under Apache-2.0 or MIT.
 * ============================================================================ */

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id_wrapper {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn new_v7() -> Self {
                Self(Uuid::now_v7())
            }
        }
    };
}

define_id_wrapper!(ProjectId);
define_id_wrapper!(SceneId);
define_id_wrapper!(AssetId);
define_id_wrapper!(CharacterId);
define_id_wrapper!(WorkflowId);
define_id_wrapper!(RenderJobId);
