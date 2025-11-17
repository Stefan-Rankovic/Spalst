/// SPDX-License-Identifier: GPL-3.0-only
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::time::Instant;

#[derive(Clone, Debug, Eq, EnumIter, PartialEq)]
pub enum MainMenuEnum {
    Browsing,
    CreatePlaythrough {
        current_input: String,
        warning_displayed_on: Option<Instant>,
    },
    LoadPlaythrough,
    Achievements,
    Settings,
    Quit,
}

impl MainMenuEnum {
    pub fn selected_default() -> Self {
        Self::selected_first()
    }
    pub fn selected_first() -> Self {
        Self::iter_no_browsing().next().unwrap()
    }
    pub fn selected_last() -> Self {
        Self::iter_no_browsing().last().unwrap()
    }

    pub fn iter_no_browsing() -> std::iter::Skip<MainMenuEnumIter> {
        Self::iter().skip(1)
    }

    pub fn as_str_debug(&self) -> &str {
        match self {
            Self::Browsing => "MainMenuEnum::Browsing",
            Self::CreatePlaythrough { .. } => "MainMenuEnum::CreatePlaythrough",
            Self::LoadPlaythrough => "MainMenuEnum::LoadPlaythrough",
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
            Self::LoadPlaythrough => "Load Game",
            Self::Achievements => "Achievements",
            Self::Settings => "Settings",
            Self::Quit => "Quit",
        }
    }
}
