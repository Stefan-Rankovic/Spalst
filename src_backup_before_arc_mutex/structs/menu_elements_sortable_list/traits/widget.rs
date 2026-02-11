//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::MenuElementsSortableList,
    traits::{AsDisplayable, MenuElements},
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::fmt::Debug;

impl<ItemId: Copy + Debug, Item: AsDisplayable + Debug> Widget
    for MenuElementsSortableList<'_, ItemId, Item>
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_elements(area, buf);
    }
}
