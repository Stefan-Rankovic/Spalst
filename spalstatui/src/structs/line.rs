//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{Styled, Widget, WidgetRef};
use core::pin::Pin;
use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect},
    style::{Style, Styled as _},
    text::Line as LineRatatui,
    widgets::Widget as _,
};

#[derive(Debug)]
pub struct Line {
    style: Style,
    pub alignment: Option<HorizontalAlignment>,
    pub text: String,
}

impl Styled for Line {
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}

impl From<LineRatatui<'_>> for Line {
    fn from(lr: LineRatatui<'_>) -> Self {
        Self {
            style: lr.style,
            alignment: lr.alignment,
            text: lr.into(),
        }
    }
}

impl From<Line> for LineRatatui<'_> {
    fn from(line: Line) -> Self {
        LineRatatui::from(line.text)
            .set_style(line.style)
            .alignment(line.alignment.unwrap_or(HorizontalAlignment::Left))
    }
}
impl From<&Line> for LineRatatui<'_> {
    fn from(line: &Line) -> Self {
        LineRatatui::from(line.text.clone())
            .set_style(line.style)
            .alignment(line.alignment.unwrap_or(HorizontalAlignment::Left))
    }
}

impl Widget for Line {
    async fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf).await;
    }
}

impl WidgetRef for Line {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            LineRatatui::from(self).render(area, buf);
        })
    }
}

impl Default for Line {
    fn default() -> Self {
        LineRatatui::default().into()
    }
}

impl Line {
    #[must_use]
    pub fn width(&self) -> usize {
        LineRatatui::from(self).width()
    }
}
