//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, MenuElementsSelectedEnum};
use strum::{EnumDiscriminants, EnumIter};
use strum_macros::{EnumCount as EnumCountMacro, EnumIs};

#[derive(Clone, Copy, Debug, EnumCountMacro, EnumIs, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
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

    fn as_str_debug(&self) -> &'static str {
        match *self {
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
    fn select_left(&self) -> Option<Self::Discriminant> {
        self.is_saves().then_some(Self::Discriminant::Name) // todo: maybe unreachable here instead of Name?
    }
    fn select_down(&self) -> Option<Self::Discriminant> {
        match *self {
            Self::Name => Some(Self::Discriminant::Playtime),
            Self::Playtime => Some(Self::Discriminant::SaveNumber),
            Self::SaveNumber => Some(Self::Discriminant::CreatedAt),
            Self::CreatedAt => Some(Self::Discriminant::LastPlayedAt),
            Self::LastPlayedAt => Some(Self::Discriminant::Note),
            Self::Note => Some(Self::Discriminant::Load),
            Self::Load | Self::Saves => None,
        }
    }
    fn select_up(&self) -> Option<Self::Discriminant> {
        match *self {
            Self::Name | Self::Saves => None,
            Self::Playtime => Some(Self::Discriminant::Name),
            Self::SaveNumber => Some(Self::Discriminant::Playtime),
            Self::CreatedAt => Some(Self::Discriminant::SaveNumber),
            Self::LastPlayedAt => Some(Self::Discriminant::CreatedAt),
            Self::Note => Some(Self::Discriminant::LastPlayedAt),
            Self::Load => Some(Self::Discriminant::Note),
        }
    }
    fn select_right(&self) -> Option<Self::Discriminant> {
        if self.is_saves() {
            None // todo: see if this can be unreachable!() instead.
        } else {
            Some(Self::Discriminant::Saves)
        }
    }
}
