//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{Widget, WidgetRef};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};

#[derive(Debug, Hash)]
pub struct Frame<'buffer> {
    pub(crate) cursor_pos: Option<Position>,
    pub(crate) area: Rect,
    pub(crate) buf: &'buffer mut Buffer,
    pub(crate) count: usize,
}

impl Frame<'_> {
    #[must_use]
    pub const fn area(&self) -> Rect {
        self.area
    }
    #[must_use]
    pub const fn buf_mut(&mut self) -> &mut Buffer {
        self.buf
    }
    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }

    pub fn set_cursor_pos<P: Into<Position>>(&mut self, position: P) {
        self.cursor_pos = Some(position.into());
    }

    pub async fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, self.buf).await;
    }
    pub async fn render_widget_ref<WR: WidgetRef>(&mut self, widget: WR, area: Rect) {
        widget.render_ref(area, self.buf).await;
    }
}
