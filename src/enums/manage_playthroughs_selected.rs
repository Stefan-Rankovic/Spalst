//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{DOWN_KEYS, LEFT_KEYS, RIGHT_KEYS, UP_KEYS},
    enums::PlaythroughsSortBy as SortBy,
    structs::{Playthrough, PlaythroughName, Playthroughs},
};
use color_eyre::eyre::{Result, bail};
use ratatui::crossterm::event::KeyCode;
use strum_macros::{EnumIs, EnumIter};

#[derive(Clone, Debug, Default, EnumIs, EnumIter, Eq, PartialEq)]
pub enum ManagePlaythroughsSelected {
    #[default]
    SortBy,
    SortAscending,
    Playthroughs {
        selected: PlaythroughName,
    },
}

impl ManagePlaythroughsSelected {
    pub fn select_decide_safe(&self, key: &KeyCode, default_playthrough: PlaythroughName) -> Self {
        match self.select_decide(key, default_playthrough) {
            Ok(output) => output,
            Err(_) => self.clone(),
        }
    }
    pub fn select_decide(
        &self,
        key: &KeyCode,
        default_playthrough: PlaythroughName,
    ) -> Result<Self> {
        let output: Self = if LEFT_KEYS.contains(key) {
            self.left()
        } else if DOWN_KEYS.contains(key) {
            self.down(default_playthrough)
        } else if UP_KEYS.contains(key) {
            self.up()
        } else if RIGHT_KEYS.contains(key) {
            self.right()
        } else {
            bail!(
                "Passed key {} to function LoadPlaythroughSelected::select_decide() but the key is not any movement key.",
                key
            );
        };
        // Ok.
        Ok(output)
    }
    pub fn left(&self) -> Self {
        match self {
            Self::SortBy => Self::SortBy,
            Self::SortAscending => Self::SortBy,
            Self::Playthroughs { .. } => unreachable!(),
        }
    }
    pub fn down(&self, default_playthrough: PlaythroughName) -> Self {
        match self {
            Self::SortBy => Self::Playthroughs {
                selected: default_playthrough,
            },
            Self::SortAscending => Self::Playthroughs {
                selected: default_playthrough,
            },
            Self::Playthroughs { .. } => unreachable!(),
        }
    }
    pub fn up(&self) -> Self {
        match self {
            Self::SortBy => Self::SortBy,
            Self::SortAscending => Self::SortAscending,
            Self::Playthroughs { .. } => unreachable!(),
        }
    }
    pub fn right(&self) -> Self {
        match self {
            Self::SortBy => Self::SortAscending,
            Self::SortAscending => Self::SortAscending,
            Self::Playthroughs { .. } => unreachable!(),
        }
    }
}
