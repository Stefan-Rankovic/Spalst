//! SPDX-License-Identifier: GPL-3.0-only
use crate::structs::App;
use color_eyre::eyre::Result;
use ratatui::crossterm::event::Event;

impl App {
    pub fn settings_handle_event(&mut self, event: Event) -> Result<()> {
        todo!()
    }
}
