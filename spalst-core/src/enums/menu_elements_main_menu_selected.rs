//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, MenuElementsSelectedEnum};
use strum::IntoDiscriminant as _;
use strum_macros::{EnumCount as EnumCountMacro, EnumDiscriminants, EnumIter};

#[derive(Clone, Copy, Debug, EnumCountMacro, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum MenuElementsMainMenuSelected {
    Continue,
    CreatePlaythrough,
    ManagePlaythroughs,
    Achievements,
    Settings,
    Quit,
}

impl EnumAsStr for MenuElementsMainMenuSelectedDiscriminants {
    fn as_str_debug(&self) -> &'static str {
        match *self {
            Self::Continue => "MenuElementsMainMenuSelected::Continue",
            Self::CreatePlaythrough => "MenuElementsMainMenuSelected::CreatePlaythrough",
            Self::ManagePlaythroughs => "MenuElementsMainMenuSelected::ManagePlaythroughs",
            Self::Achievements => "MenuElementsMainMenuSelected::Achievements",
            Self::Settings => "MenuElementsMainMenuSelected::Settings",
            Self::Quit => "MenuElementsMainMenuSelected::Quit",
        }
    }

    fn as_str_user(&self) -> &'static str {
        match *self {
            Self::Continue => "Continue",
            Self::CreatePlaythrough => "Create Playthrough",
            Self::ManagePlaythroughs => "Manage Playthroughs",
            Self::Achievements => "Achievements",
            Self::Settings => "Settings",
            Self::Quit => "Quit",
        }
    }
}
impl EnumAsStr for MenuElementsMainMenuSelected {
    fn as_str_debug(&self) -> &'static str {
        self.discriminant().as_str_debug()
    }
    fn as_str_user(&self) -> &'static str {
        self.discriminant().as_str_user()
    }
}

impl MenuElementsSelectedEnum for MenuElementsMainMenuSelected {
    fn select_left(&self) -> Option<Self::Discriminant> {
        None
    }

    fn select_down(&self) -> Option<Self::Discriminant> {
        match *self {
            Self::Continue => Some(Self::Discriminant::CreatePlaythrough),
            Self::CreatePlaythrough => Some(Self::Discriminant::ManagePlaythroughs),
            Self::ManagePlaythroughs => Some(Self::Discriminant::Achievements),
            Self::Achievements => Some(Self::Discriminant::Settings),
            Self::Settings => Some(Self::Discriminant::Quit),
            Self::Quit => None,
        }
    }

    fn select_up(&self) -> Option<Self::Discriminant> {
        match *self {
            Self::Continue => None,
            Self::CreatePlaythrough => Some(Self::Discriminant::Continue),
            Self::ManagePlaythroughs => Some(Self::Discriminant::CreatePlaythrough),
            Self::Achievements => Some(Self::Discriminant::ManagePlaythroughs),
            Self::Settings => Some(Self::Discriminant::Achievements),
            Self::Quit => Some(Self::Discriminant::Settings),
        }
    }

    fn select_right(&self) -> Option<Self::Discriminant> {
        None
    }
}
