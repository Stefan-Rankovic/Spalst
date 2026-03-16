//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{buffer::Buffer, layout::Rect};

pub(crate) trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer) -> impl Future<Output = ()>;
}
