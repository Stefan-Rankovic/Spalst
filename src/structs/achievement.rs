/// SPDX-License-Identifier: GPL-3.0-only
use crate::enums::{AchievementId, Rarity};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Achievement {
    id: AchievementId,
}

impl From<AchievementId> for Achievement {
    fn from(id: AchievementId) -> Self {
        Self { id }
    }
}

impl Achievement {
    pub fn name_debug(&self) -> &str {
        self.id.as_str_debug()
    }

    pub fn name_user(&self) -> &str {
        self.id.as_str_user()
    }
    pub fn description(&self) -> &str {
        match self.id {
            AchievementId::EnterPlaythrough => "Enter your first Playthrough.",
        }
    }
    pub fn rarity(&self) -> Rarity {
        match self.id {
            AchievementId::EnterPlaythrough => Rarity::Common,
        }
    }
}
