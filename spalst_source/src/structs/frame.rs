//! SPDX-License-Identifier: MIT

use crate::traits::Renderable;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};

/// For any documentation, check [docs.rs](https://docs.rs/ratatui-core/0.1.0/ratatui_core/terminal/struct.Frame.html).
#[derive(Debug, Hash)]
pub struct Frame<'buffer> {
    cursor_position: Option<Position>,
    viewport_area: Rect,
    buffer: &'buffer mut Buffer,
    count: usize,
}

impl<'buffer> Frame<'buffer> {
    #[must_use]
    pub const fn new(
        cursor_position: Option<Position>,
        viewport_area: Rect,
        buffer: &'buffer mut Buffer,
        count: usize,
    ) -> Self {
        Self {
            cursor_position,
            viewport_area,
            buffer,
            count,
        }
    }

    pub const fn get_cursor_position(&self) -> Option<Position> {
        self.cursor_position
    }

    pub const fn area(&self) -> Rect {
        self.viewport_area
    }

    #[deprecated = "use `area()` instead"]
    pub const fn size(&self) -> Rect {
        self.viewport_area
    }

    pub async fn render_renderable<R: Renderable>(
        &mut self,
        widget: R,
        area: Rect,
    ) {
        widget.basic_render(area, self.buffer).await;
    }

    pub fn set_cursor_position<P: Into<Position>>(
        &mut self,
        position: P,
    ) {
        self.cursor_position = Some(position.into());
    }

    #[deprecated = "use `set_cursor_position((x, y))` instead which takes `impl Into<Position>`"]
    pub fn set_cursor(
        &mut self,
        x: u16,
        y: u16,
    ) {
        self.set_cursor_position(Position { x, y });
    }

    pub const fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    pub const fn count(&self) -> usize {
        self.count
    }
}
