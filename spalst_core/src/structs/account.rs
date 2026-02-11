//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::{Achievement, AchievementQueue, Playthroughs, SaveId},
    traits::{Loadable, LoadableSafe, Saveable},
};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub playthroughs: Playthroughs,
    next_save_id: SaveId,
    achievements: HashSet<Achievement>,
    /// May get removed if I don't like it. todo. Probably going to get moved to Settings.
    fps: u16,
}

impl Default for Account {
    fn default() -> Self {
        // Remove this if account.fps is also removed (just derive Default)
        Self {
            playthroughs: Playthroughs::default(),
            next_save_id: 0.into(),
            achievements: HashSet::new(),
            fps: 60,
        }
    }
}

impl Loadable for Account {}

impl LoadableSafe for Account {}

impl Saveable for Account {}

impl Account {
    pub const fn fps(&self) -> &u16 {
        &self.fps
    }

    pub fn award_achievement<A: Into<Achievement>>(
        &mut self,
        achievement_into: A,
        achievement_queue: &mut AchievementQueue,
    ) {
        // Define the achievement
        let achievement: Achievement = achievement_into.into();
        // If this achievement was a new one, add it to the displaying queue, otherwise debug!() and
        // continue.
        if self.achievements.insert(achievement) {
            achievement_queue.queue_achievement(achievement);
        } else {
            debug!(
                "Tried awarding the user the achievement {}, but they already have it.",
                achievement.name_debug()
            );
        }
    }
}
