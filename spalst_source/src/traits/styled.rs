//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::style::Style;

/// Applies things like color to literally anything.
///
/// There exist no regulations on how the struct implementing this trait will implement it.
pub trait Styled {
    /// Checks whether the current style (gotten through `Styled::get_style()`) is equal to `Style::default()`.
    fn has_style(&self) -> bool {
        self.get_style() == Style::default()
    }
    fn get_style(&self) -> Style;
    fn set_style(
        &mut self,
        new_style: Style,
    ) -> Style;
}
