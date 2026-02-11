//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::MenuElementList, traits::AsDisplayable};
use core::fmt::Debug;
use ratatui::style::Style;
use spalstatui::traits::Styled;

impl<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> Styled for MenuElementList<ItemId, Item>
{
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
