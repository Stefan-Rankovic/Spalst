//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::widgets::{Block, BorderType, Padding};

/// Block padding.
const PADDING: u16 = 1;

/// Constructs a block with useful defaults.
pub const fn block() -> Block<'static> {
    Block::bordered()
        .padding(Padding::uniform(PADDING))
        .border_type(BorderType::Rounded)
}

/// The dimensions of the `Block` provided by `block()`.
/// Width and then height.
pub const fn block_dimensions() -> (u16, u16) {
    (2 + PADDING * 2, 2 + PADDING * 2)
}
