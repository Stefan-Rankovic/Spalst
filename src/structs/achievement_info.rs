/// SPDX-License-Identifier: GPL-3.0-only
use crate::enums::{AchievementId, Rarity};

pub struct AchievementInfo {
    name: &'static str,
    description: &'static str,
    rarity: Rarity,
}

impl AchievementInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn description(&self) -> &'static str {
        self.description
    }
    pub fn rarity(&self) -> Rarity {
        self.rarity
    }

    pub fn from_id(id: AchievementId) -> Self {
        match id {
            AchievementId::Hello => Self {
                name: "Hello!",
                description: "Enter the game for the first time.",
                rarity: Rarity::Common,
            },
        }
    }
}
