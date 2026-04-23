//! SPDX-License-Identifier: MIT

use ratatui::{buffer::Buffer, layout::Rect};

/// For any documentation, check [docs.rs](https://docs.rs/ratatui-core/0.1.0/ratatui_core/terminal/struct.CompletedFrame.html).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CompletedFrame<'buffer> {
    /// The buffer that was used to draw the last frame.
    pub buffer: &'buffer Buffer,
    /// The size of the last frame.
    pub area: Rect,
    /// The frame count indicating the sequence number of this frame.
    pub count: usize,
}
