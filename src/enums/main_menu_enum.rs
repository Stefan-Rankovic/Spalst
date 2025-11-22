//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::ManagePlaythroughsMenuDiscriminants;
use crate::enums::{ManagePlaythroughsMenu, ManagePlaythroughsSelected, PlaythroughsSortBy};
use strum::IntoEnumIterator;
use strum_macros::EnumDiscriminants;
use strum_macros::EnumIter;
use tokio::time::Instant;

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum MainMenuEnum {
    Browsing,
    CreatePlaythrough {
        current_input: String,
        warning_displayed_on: Option<Instant>,
    },
    ManagePlaythroughs(ManagePlaythroughsMenu),
    Achievements,
    Settings,
    Quit,
}

impl From<MainMenuEnumDiscriminants> for MainMenuEnum {
    fn from(discriminant: MainMenuEnumDiscriminants) -> Self {
        match discriminant {
            MainMenuEnumDiscriminants::Browsing => Self::Browsing,
            MainMenuEnumDiscriminants::CreatePlaythrough => Self::CreatePlaythrough {
                current_input: "".to_string(),
                warning_displayed_on: None,
            },
            MainMenuEnumDiscriminants::ManagePlaythroughs => {
                Self::ManagePlaythroughs(ManagePlaythroughsMenu::default())
            }
            MainMenuEnumDiscriminants::Achievements => Self::Achievements,
            MainMenuEnumDiscriminants::Settings => Self::Settings,
            MainMenuEnumDiscriminants::Quit => Self::Quit,
        }
    }
}

impl MainMenuEnumDiscriminants {
    pub fn selected_default() -> Self {
        Self::selected_first()
    }
    pub fn selected_first() -> Self {
        Self::iter_no_browsing().next().unwrap()
    }
    pub fn selected_last() -> Self {
        Self::iter_no_browsing().last().unwrap()
    }

    pub fn iter_no_browsing() -> std::iter::Skip<MainMenuEnumDiscriminantsIter> {
        Self::iter().skip(1)
    }

    pub fn as_str_debug(&self) -> &str {
        match self {
            Self::Browsing => "MainMenuEnum::Browsing",
            Self::CreatePlaythrough { .. } => "MainMenuEnum::CreatePlaythrough",
            Self::ManagePlaythroughs { .. } => "MainMenuEnum::LoadPlaythrough",
            Self::Achievements => "MainMenuEnum::Achievements",
            Self::Settings => "MainMenuEnum::Settings",
            Self::Quit => "MainMenuEnum::Quit",
        }
    }
    pub fn as_str_user(&self) -> &str {
        match self {
            Self::Browsing => panic!(
                "MainMenuEnum::user_to_str() should never be called on {}.",
                Self::Browsing.as_str_debug()
            ),
            Self::CreatePlaythrough { .. } => "New Game",
            Self::ManagePlaythroughs { .. } => "Load Game",
            Self::Achievements => "Achievements",
            Self::Settings => "Settings",
            Self::Quit => "Quit",
        }
    }
}
