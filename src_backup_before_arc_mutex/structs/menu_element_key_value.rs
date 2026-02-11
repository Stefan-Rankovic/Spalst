//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::MenuElement;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use std::any::Any;

#[derive(Clone, Copy, Debug)]
pub struct MenuElementKeyValue<'data> {
    selectable: bool,
    selected: bool,

    key: &'data str,
    value: &'data str,
}

impl<'data> MenuElement for MenuElementKeyValue<'data> {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'data> Widget for MenuElementKeyValue<'data> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("{}: {}", self.key, self.value)).render(area, buf)
    }
}
