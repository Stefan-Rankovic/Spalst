//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::MenuElementsSortableList, traits::AsDisplayable};
use ratatui::style::Style;
use spalstatui::traits::Styled;
use std::fmt::Debug;

impl<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> Styled for MenuElementsSortableList<ItemId, Item>
{
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
