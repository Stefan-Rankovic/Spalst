//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::ACHIEVEMENT_DISPLAY_TIME,
    enums::{MainMenuEnum, ManagePlaythroughsMenu, VerticalAlignment},
    structs::{Achievement, App},
    utils::{create_block, create_popup},
};
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

impl App {
    pub fn display(&self, terminal: &mut AsyncTerminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        // Ok.
        Ok(())
    }
}
