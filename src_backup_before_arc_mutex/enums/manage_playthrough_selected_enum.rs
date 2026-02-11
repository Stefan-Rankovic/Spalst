//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::SelectMenuEnum;
use strum::IntoDiscriminant;
use strum_macros::{EnumDiscriminants, EnumIs, EnumIter};

///todo: remove this when possible
#[deprecated = "Use MenuElementsManagePlaythroughSelected instead."]
#[derive(Clone, Copy, Debug, EnumDiscriminants, EnumIs, EnumIter)]
pub enum ManagePlaythroughSelectedEnum {
    Name,

    Playtime,
    SaveNumber,

    CreatedAt,
    LastPlayedAt,

    Note,

    Load,

    Saves,
}

impl SelectMenuEnum for ManagePlaythroughSelectedEnum {
    fn select_left(&self) -> Option<Self> {
        if self.is_saves() {
            Some(Self::Name) // todo: see if this can be unreachable!() instead
        } else {
            None
        }
    }
    fn select_down(&self) -> Option<Self> {
        match self.discriminant() {
            ManagePlaythroughSelectedEnumDiscriminants::Name => Some(Self::Playtime),
            ManagePlaythroughSelectedEnumDiscriminants::Playtime => Some(Self::SaveNumber),
            ManagePlaythroughSelectedEnumDiscriminants::SaveNumber => Some(Self::CreatedAt),
            ManagePlaythroughSelectedEnumDiscriminants::CreatedAt => Some(Self::LastPlayedAt),
            ManagePlaythroughSelectedEnumDiscriminants::LastPlayedAt => Some(Self::Note),
            ManagePlaythroughSelectedEnumDiscriminants::Note => Some(Self::Load),
            ManagePlaythroughSelectedEnumDiscriminants::Load => None,
            ManagePlaythroughSelectedEnumDiscriminants::Saves => None,
        }
    }
    fn select_up(&self) -> Option<Self> {
        match self.discriminant() {
            ManagePlaythroughSelectedEnumDiscriminants::Name => None,
            ManagePlaythroughSelectedEnumDiscriminants::Playtime => Some(Self::Name),
            ManagePlaythroughSelectedEnumDiscriminants::SaveNumber => Some(Self::Playtime),
            ManagePlaythroughSelectedEnumDiscriminants::CreatedAt => Some(Self::SaveNumber),
            ManagePlaythroughSelectedEnumDiscriminants::LastPlayedAt => Some(Self::CreatedAt),
            ManagePlaythroughSelectedEnumDiscriminants::Note => Some(Self::LastPlayedAt),
            ManagePlaythroughSelectedEnumDiscriminants::Load => Some(Self::Note),
            ManagePlaythroughSelectedEnumDiscriminants::Saves => None,
        }
    }
    fn select_right(&self) -> Option<Self> {
        if self.is_saves() {
            None
        } else {
            Some(Self::Saves)
        }
    }
}
