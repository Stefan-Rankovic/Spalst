//! SPDX-License-Identifier: GPL-3.0-only

use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Default, EnumIter, Eq, PartialEq)]
pub enum PlaythroughsSortBy {
    #[default]
    LastPlayedAt,
    CreatedAt,
    Name,
    SaveNumber,
    Playtime,
}

impl PlaythroughsSortBy {
    pub fn as_str_debug(&self) -> &str {
        match self {
            Self::LastPlayedAt => "PlaythroughsSortBy::LastPlayedAt",
            Self::CreatedAt => "PlaythroughsSortBy::CreatedAt",
            Self::Name => "PlaythroughsSortBy::Name",
            Self::SaveNumber => "PlaythroughsSortBy::SaveNumber",
            Self::Playtime => "PlaythroughsSortBy::Playtime",
        }
    }
    pub fn as_str_user(&self) -> &str {
        match self {
            Self::LastPlayedAt => "Last played",
            Self::CreatedAt => "Creation time",
            Self::Name => "Name",
            Self::SaveNumber => "Number of saves",
            Self::Playtime => "Playtime",
        }
    }
}
