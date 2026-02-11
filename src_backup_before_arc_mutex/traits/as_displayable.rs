//! SPDX-License-Identifier: GPL-3.0-only

use crate::types::ItemBlockInfo;
use ratatui::text::Text;

pub trait AsDisplayable {
    fn as_displayable<'t, 'b>(&'b self, selected: bool) -> (Text<'t>, Option<ItemBlockInfo<'b>>);
}
