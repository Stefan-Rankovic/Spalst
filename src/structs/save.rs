//! SPDX-License-Identifier: GPL-3.0-only
use crate::structs::Game;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    game: Game,
    note: Option<String>,
    created_at: DateTime<Utc>,
    last_played_at: Option<DateTime<Utc>>,
    auto_save: bool,
}

impl Save {
    pub fn initial() -> Self {
        Self {
            game: Game::default(),
            note: Some(String::from(
                "The initial save. This is the only save automatically created by the game that is not marked as an auto save.",
            )),
            created_at: Utc::now(),
            last_played_at: None,
            auto_save: false,
        }
    }
}
