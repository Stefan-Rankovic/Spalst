//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuElementListEnum,
    traits::{MenuElement, MenuElements, MenuElementsSelectedEnum},
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug)]
pub struct Menu<'data> {
    // pub current: Box<dyn MenuElement>,
    pub current: MenuElementListEnum<'data>,
}

impl<'data> Widget for Menu<'data> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.current.render(area, buf);
    }
}

impl<'data> From<MenuElementListEnum<'data>> for Menu<'data> {
    fn from(element: MenuElementListEnum<'data>) -> Self {
        Self { current: element }
    }
}

impl<'data> Menu<'data> {
    pub fn new(menu_element: MenuElementListEnum<'data>) -> Self {
        Self {
            current: menu_element,
        }
    }
}
