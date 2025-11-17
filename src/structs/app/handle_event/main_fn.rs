/// SPDX-License-Identifier: GPL-3.0-only
use crate::{enums::MainMenuEnum, structs::App};
use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, crossterm::event::Event};

impl App {
    pub fn handle_event(&mut self, event: Event, terminal: &mut DefaultTerminal) -> Result<()> {
        if let Event::Resize(..) = event {
            self.display(terminal)?;
            return Ok(());
        };
        match self.menu().current() {
            MainMenuEnum::Browsing => self.browsing_handle_event(event),
            MainMenuEnum::CreatePlaythrough { .. } => self.create_playthrough_handle_event(event),
            MainMenuEnum::LoadPlaythrough => self.load_playthrough_handle_event(event),
            MainMenuEnum::Achievements => self.achievements_handle_event(event),
            MainMenuEnum::Settings => self.settings_handle_event(event),
            MainMenuEnum::Quit => Ok(()),
        }
    }
}
