//! SPDX-License-Identifier: GPL-3.0-only

use crate::types::ItemBlockInfo;
use ratatui::text::Text;

pub trait AsDisplayable {
    /// todo: maybe refactor this to use spalstatui's Block?
    fn as_displayable<'text>(&'_ self, selected: bool) -> (Text<'text>, Option<ItemBlockInfo<'_>>);
}
