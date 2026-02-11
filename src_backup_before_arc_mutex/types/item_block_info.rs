//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::widgets::Block;

/// The block, it's base height (with no text in it), and then base width (with no text in it)..
pub type ItemBlockInfo<'b> = (Block<'b>, (usize, usize));
