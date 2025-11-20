//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::ENTITY_TEMPLATES_PATH,
    enums::AchievementId,
    structs::{
        Achievement, AchievementQueue, Playthrough, PlaythroughId, PlaythroughName, Save, SaveId,
    },
    traits::{Loadable, LoadableSafe, Saveable},
    utils::convert_path,
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::{self, create_dir},
    path::{Path, PathBuf},
};
use tokio::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    game_initialized: bool,
    pub playthroughs: HashMap<PlaythroughName, Playthrough>,
    next_save_id: SaveId,
    achievements: HashSet<Achievement>,
    fps: u16, // may get removed if I don't like it
}

impl Default for Account {
    fn default() -> Self {
        // Remove this if account.fps is also removed
        Self {
            game_initialized: false,
            playthroughs: HashMap::new(),
            next_save_id: SaveId(0),
            achievements: HashSet::new(),
            fps: 60,
        }
    }
}

impl Loadable for Account {}

impl LoadableSafe for Account {}

impl Saveable for Account {}

impl Account {
    pub fn fps(&self) -> u16 {
        self.fps
    }

    pub fn initialize_game(&mut self, game_path: &Path) -> Result<()> {
        if self.game_initialized {
            return Ok(());
        };
        if !game_path.is_dir() {
            bail!("Game path passed to Account.initialize_game() is not a directory");
        };
        let entity_templates_path: &Path = &game_path.join(ENTITY_TEMPLATES_PATH);
        if !entity_templates_path.try_exists()? {
            create_dir(entity_templates_path)?;
        } else if !entity_templates_path.is_dir() {
            bail!(
                "{} exists but is not a directory.",
                entity_templates_path.display()
            );
        };
        self.game_initialized = true;
        // Ok.
        Ok(())
    }

    pub fn award_achievement<A>(&mut self, achievement: A, achievement_queue: &mut AchievementQueue)
    where
        A: Into<Achievement>,
    {
        // Define the achievement
        let achievement: Achievement = achievement.into();
        // If this achievement was a new one, add it to the displaying queue, otherwise debug!() and
        // continue.
        if self.achievements.insert(achievement) {
            achievement_queue.queue_achievement(achievement);
        } else {
            debug!(
                "Tried awarding the user the achievement {}, but they already have it.",
                achievement.name_debug()
            );
        };
    }
}
