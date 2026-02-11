//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, MenuElementsSelectedEnum};
use strum_macros::{EnumCount as EnumCountMacro, EnumIs};

#[derive(Clone, Copy, Debug, EnumCountMacro, EnumIs)]
pub enum MenuElementsManagePlaythroughSelected {
    Name,

    Playtime,
    SaveNumber,

    CreatedAt,
    LastPlayedAt,

    Note,

    Load,

    Saves,
}

impl EnumAsStr for MenuElementsManagePlaythroughSelected {
    fn as_str_user(&self) -> &'static str {
        unreachable!();
        // match self {
        //     Self::Name => "Name",
        //     Self::Playtime => "Playtime",
        //     Self::SaveNumber => "Save Number",
        //     Self::CreatedAt => "Created At",
        //     Self::LastPlayedAt => "Last Played At",
        //     Self::Note => "Note",
        //     Self::Load => "Load",
        //     Self::Saves => "Saves",
        // }
    }

    fn as_str_debug(&self) -> &str {
        match self {
            Self::Name => "MenuElementsManagePlaythroughSelected::Name",
            Self::Playtime => "MenuElementsManagePlaythroughSelected::Playtime",
            Self::SaveNumber => "MenuElementsManagePlaythroughSelected::SaveNumber",
            Self::CreatedAt => "MenuElementsManagePlaythroughSelected::CreatedAt",
            Self::LastPlayedAt => "MenuElementsManagePlaythroughSelected::LastPlayedAt",
            Self::Note => "MenuElementsManagePlaythroughSelected::Note",
            Self::Load => "MenuElementsManagePlaythroughSelected::Load",
            Self::Saves => "MenuElementsManagePlaythroughSelected::Saves",
        }
    }
}

impl MenuElementsSelectedEnum for MenuElementsManagePlaythroughSelected {
    fn select_left(&self) -> Option<Self> {
        if self.is_saves() {
            Some(Self::Name) // todo: see if this can be unreachable!() instead
        } else {
            None
        }
    }
    fn select_down(&self) -> Option<Self> {
        match self {
            Self::Name => Some(Self::Playtime),
            Self::Playtime => Some(Self::SaveNumber),
            Self::SaveNumber => Some(Self::CreatedAt),
            Self::CreatedAt => Some(Self::LastPlayedAt),
            Self::LastPlayedAt => Some(Self::Note),
            Self::Note => Some(Self::Load),
            Self::Load => None,
            Self::Saves => None,
        }
    }
    fn select_up(&self) -> Option<Self> {
        match self {
            Self::Name => None,
            Self::Playtime => Some(Self::Name),
            Self::SaveNumber => Some(Self::Playtime),
            Self::CreatedAt => Some(Self::SaveNumber),
            Self::LastPlayedAt => Some(Self::CreatedAt),
            Self::Note => Some(Self::LastPlayedAt),
            Self::Load => Some(Self::Note),
            Self::Saves => None,
        }
    }
    fn select_right(&self) -> Option<Self> {
        if self.is_saves() {
            None // todo: see if this can be unreachable!() instead.
        } else {
            Some(Self::Saves)
        }
    }
}
