/// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum AchievementId {
    EnterPlaythrough,
}

impl AchievementId {
    pub fn as_str_debug(&self) -> &str {
        match self {
            Self::EnterPlaythrough => "AchievementId::EnteredPlaythrough",
        }
    }
    pub fn as_str_user(&self) -> &str {
        match self {
            Self::EnterPlaythrough => "And so the Journey Begins",
        }
    }
}
