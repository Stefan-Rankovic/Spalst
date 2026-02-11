//! SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub enum AchievementId {
    EnterPlaythrough,
}

impl AchievementId {
    pub const fn as_str_debug(self) -> &'static str {
        match self {
            Self::EnterPlaythrough => "AchievementId::EnteredPlaythrough",
        }
    }
    pub const fn as_str_user(self) -> &'static str {
        match self {
            Self::EnterPlaythrough => "And so the Journey Begins",
        }
    }
}
