//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::widgets::Block;

/// The block, it's base height (with no text in it), and then base width (with no text in it).
///
/// todo: maybe refactor this to use spalstatui's Block?
pub type ItemBlockInfo<'block> = (Block<'block>, (usize, usize));
