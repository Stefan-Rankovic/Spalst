/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    enums::MainMenuEnum,
    structs::{App, Playthrough, PlaythroughName},
};
use color_eyre::eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode};
use tokio::time::Instant;

impl App {
    pub fn create_playthrough_handle_event(&mut self, event: Event) -> Result<()> {
        let MainMenuEnum::CreatePlaythrough {
            current_input,
            warning_displayed_on,
        } = self.menu().current()
        else {
            unreachable!();
        };
        if let Event::Key(key) = event {
            if warning_displayed_on.is_some() {
                // Ok.
                return Ok(());
            };
            match key.code {
                KeyCode::Backspace => {
                    if !current_input.is_empty() {
                        let current_input: String = current_input.to_string();
                        let warning_displayed_on: Option<Instant> = *warning_displayed_on;
                        self.menu_mut().set_same(MainMenuEnum::CreatePlaythrough {
                            current_input: current_input[..current_input.len() - 1].to_string(),
                            warning_displayed_on,
                        })?;
                    };
                }
                KeyCode::Char(c) => {
                    let mut current_input: String = current_input.to_string();
                    current_input.push(c);
                    let warning_displayed_on: Option<Instant> = *warning_displayed_on;
                    self.menu_mut().set_same(MainMenuEnum::CreatePlaythrough {
                        current_input,
                        warning_displayed_on,
                    })?;
                }
                KeyCode::Esc => self.menu_mut().browse(),
                KeyCode::Enter => {
                    let current_input: String = current_input.clone();
                    // Check if there's already a playthrough with the same name. If there is,
                    // display a warning.
                    if self
                        .account
                        .playthroughs
                        .contains_key(&PlaythroughName(current_input.clone()))
                    {
                        self.menu_mut().set_same(MainMenuEnum::CreatePlaythrough {
                            current_input,
                            warning_displayed_on: Some(Instant::now()),
                        })?;
                    } else {
                        self.menu_mut().browse();
                        self.account
                            .playthroughs
                            .insert(PlaythroughName(current_input), Playthrough::default());
                    };
                }
                _ => {}
            };
        };
        // Ok.
        Ok(())
    }
}
