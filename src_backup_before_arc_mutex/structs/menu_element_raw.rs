//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::traits::MenuElement;

#[derive(Copy, Clone, Debug)]
pub struct MenuElementRaw<'text> {
    selectable: bool,
    selected: bool,

    pub text: &'text str,
}

impl<'text> MenuElementRaw<'text> {
    pub fn new(selectable: bool, selected: bool, text: &'text str) -> Self {
        Self {
            selectable,
            selected,
            text,
        }
    }
}

impl<'text> MenuElement for MenuElementRaw<'text> {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected
    }
}

impl Widget for MenuElementRaw<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.text.render(area, buf);
    }
}
