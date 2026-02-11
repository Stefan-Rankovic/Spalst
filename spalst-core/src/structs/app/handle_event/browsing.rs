//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{DOWN_KEYS, ESCAPE_KEYS, RIGHT_KEYS, UP_KEYS},
    enums::{MainMenuEnum, MainMenuEnumDiscriminants, Select},
    structs::App,
};
use color_eyre::eyre::{OptionExt, Result};
use ratatui::crossterm::event::{Event, KeyCode};
use strum::IntoDiscriminant;

impl App {
    pub fn browsing_handle_event(&mut self, event: Event) -> Result<()> {
        if self.menu().current().discriminant() != MainMenuEnumDiscriminants::Browsing {
            unreachable!();
        };
        // By how much options do PageUp and PageDown scroll
        let page_keys_change_by: u8 = 3;
        // Handle events
        if let Event::Key(key) = event {
            match key.code {
                code if UP_KEYS.contains(&code) => self.menu_mut().select(Select::Previous)?,
                code if DOWN_KEYS.contains(&code) => self.menu_mut().select(Select::Next)?,
                KeyCode::PageUp => {
                    for _ in 0..page_keys_change_by {
                        self.menu_mut().select(Select::Previous)?
                    }
                }
                KeyCode::PageDown => {
                    for _ in 0..page_keys_change_by {
                        self.menu_mut().select(Select::Next)?
                    }
                }
                KeyCode::Home => self
                    .menu_mut()
                    .select(Select::Direct(MainMenuEnumDiscriminants::selected_first()))?,
                code if ESCAPE_KEYS.contains(&code) || code == KeyCode::End => self
                    .menu_mut()
                    .select(Select::Direct(MainMenuEnumDiscriminants::selected_last()))?,
                code if RIGHT_KEYS.contains(&code) || code == KeyCode::Enter => {
                    let selected: MainMenuEnumDiscriminants =
                        self.menu().selected().ok_or_eyre("probably_unreachable")?;
                    self.menu_mut().set(selected);
                }
                _ => {}
            };
        };
        // Ok.
        Ok(())
    }
}
