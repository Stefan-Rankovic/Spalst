//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::Styled;
use ratatui::{buffer::Buffer, layout::Rect};

/// You can now see things on the screen! Revolutionary.
///
/// Why not `Widget` or `WidgetRef`? Because they have a `render()` method, while this is
/// `basic_render()`.
pub trait Renderable: Styled {
    fn basic_render(
        &self,
        area: Rect,
        buf: &mut Buffer,
    );
}
