/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    enums::{MainMenuEnum, Select},
    structs::App,
};
use color_eyre::eyre::{OptionExt, Result};
use ratatui::crossterm::event::{Event, KeyCode};

impl App {
    pub fn browsing_handle_event(&mut self, event: Event) -> Result<()> {
        if *self.menu().current() != MainMenuEnum::Browsing {
            unreachable!();
        };
        // By how much options do PageUp and PageDown scroll
        let page_keys_change_by: u8 = 3;
        // Handle events
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                    self.menu_mut().select(Select::Previous)?
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                    self.menu_mut().select(Select::Next)?
                }
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
                    .select(Select::Direct(MainMenuEnum::selected_first()))?,
                KeyCode::End => self
                    .menu_mut()
                    .select(Select::Direct(MainMenuEnum::selected_last()))?,
                KeyCode::Char('q') => {
                    if *self.menu().selected().ok_or_eyre(format!("self.menu().current() magically changed from MainMenuEnum::Browsing to {} in 10 lines of code.", self.menu().current().as_str_debug()))? == MainMenuEnum::Quit {
                        self.menu_mut().set(MainMenuEnum::Quit);
                    } else {
                        self.menu_mut().select(Select::Direct(MainMenuEnum::Quit))?;
                    };
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                    let selected: MainMenuEnum = self.menu().selected().ok_or_eyre(format!("self.menu().current() magically changed from MainMenuEnum::Browsing to {} in 10 lines of code.", self.menu().current().as_str_debug()))?.clone();
                    self.menu_mut().set(selected);
                }
                _ => {}
            };
        };
        // Ok.
        Ok(())
    }
}
