//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::style::{Color, Modifier, Style};

pub trait Styled {
    /// Returns the current `Style`.
    fn style(&self) -> Style;
    /// Sets a new `Style`.
    fn set_style(&mut self, style: Style);

    /// Sets a new background.
    fn background(&mut self, color: Color) {
        self.set_style(self.style().bg(color));
    }
    /// Sets a new foreground.
    fn foreground(&mut self, color: Color) {
        self.set_style(self.style().fg(color));
    }
    /// Resets the `Style`.
    fn reset(&mut self) {
        self.set_style(Style::reset());
    }
    /// Adds a new `Modifier`.
    fn add_modifier(&mut self, modifier: Modifier) {
        self.set_style(self.style().add_modifier(modifier));
    }
    /// Removes an existing `Modifier`.
    fn remove_modifier(&mut self, modifier: Modifier) {
        self.set_style(self.style().remove_modifier(modifier));
    }

    /// Adds the `BOLD` `Modifier`.
    fn bold(&mut self) {
        self.add_modifier(Modifier::BOLD);
    }
    /// Adds the `ITALIC` `Modifier`.
    fn italic(&mut self) {
        self.add_modifier(Modifier::ITALIC);
    }
    /// Adds the `DIM` `Modifier`.
    fn dim(&mut self) {
        self.add_modifier(Modifier::DIM);
    }
}
