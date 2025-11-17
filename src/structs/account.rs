/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::ENTITY_TEMPLATES_PATH,
    enums::AchievementId,
    structs::{Playthrough, PlaythroughId, PlaythroughName, Save, SaveId},
    traits::{Loadable, LoadableSafe, Saveable},
    utils::convert_path,
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, create_dir},
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    game_initialized: bool,
    pub playthroughs: HashMap<PlaythroughName, Playthrough>,
    next_save_id: SaveId,
    achievements: Vec<AchievementId>,
    fps: u16, // may get removed if I don't like it
}

impl Default for Account {
    fn default() -> Self {
        // Remove this if account.fps is also removed
        Self {
            game_initialized: false,
            playthroughs: HashMap::new(),
            next_save_id: SaveId(0),
            achievements: Vec::new(),
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

    pub fn initialize_game(&self, game_path: &Path) -> Result<()> {
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
        // Ok.
        Ok(())
    }
}
