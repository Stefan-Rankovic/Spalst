//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, SortMethod};
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Default, EnumIter, Eq, PartialEq)]
pub enum PlaythroughsSortMethod {
    Name,

    Playtime,
    SaveNumber,

    CreatedAt,
    #[default]
    LastPlayedAt,
}

impl SortMethod for PlaythroughsSortMethod {}

impl EnumAsStr for PlaythroughsSortMethod {
    fn as_str_debug(&self) -> &str {
        match self {
            Self::LastPlayedAt => "PlaythroughsSortBy::LastPlayedAt",
            Self::CreatedAt => "PlaythroughsSortBy::CreatedAt",
            Self::Name => "PlaythroughsSortBy::Name",
            Self::SaveNumber => "PlaythroughsSortBy::SaveNumber",
            Self::Playtime => "PlaythroughsSortBy::Playtime",
        }
    }
    fn as_str_user(&self) -> &str {
        match self {
            Self::LastPlayedAt => "Last played",
            Self::CreatedAt => "Creation time",
            Self::Name => "Name",
            Self::SaveNumber => "Number of saves",
            Self::Playtime => "Playtime",
        }
    }
}
