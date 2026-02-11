//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, SortMethod};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Default, Deserialize, EnumIter, Eq, PartialEq, Serialize)]
pub enum PlaythroughsSortMethod {
    Name,

    Playtime,
    SaveNumber,

    CreatedAgo, // todo: finish the change from At to Ago
    #[default]
    LastPlayedAgo, // todo: finish the change from At to Ago
}

impl SortMethod for PlaythroughsSortMethod {}

impl EnumAsStr for PlaythroughsSortMethod {
    fn as_str_debug(&self) -> &'static str {
        match *self {
            Self::LastPlayedAgo => "PlaythroughsSortBy::LastPlayedAgo",
            Self::CreatedAgo => "PlaythroughsSortBy::CreatedAgo",
            Self::Name => "PlaythroughsSortBy::Name",
            Self::SaveNumber => "PlaythroughsSortBy::SaveNumber",
            Self::Playtime => "PlaythroughsSortBy::Playtime",
        }
    }
    fn as_str_user(&self) -> &'static str {
        match *self {
            Self::LastPlayedAgo => "Last played ago",
            Self::CreatedAgo => "Created ago",
            Self::Name => "Name",
            Self::SaveNumber => "Number of saves",
            Self::Playtime => "Playtime",
        }
    }
}
