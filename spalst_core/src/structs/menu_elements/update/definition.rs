//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::MenuElements;
use ratatui::style::Style;
use spalstatui::structs::Block;

#[derive(Debug)]
pub struct Update {
    pub(in crate::structs::menu_elements::update) selectable: bool,
    pub(in crate::structs::menu_elements::update) selected:
        Option<<Self as MenuElements>::Elements>,

    pub(in crate::structs::menu_elements::update) block: Option<Block>,

    pub(in crate::structs::menu_elements::update) style: Style,
}
