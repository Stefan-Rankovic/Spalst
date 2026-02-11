//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::Line,
    traits::{Styled, Widget, WidgetRef},
};
use core::pin::Pin;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::{border, merge::MergeStrategy},
    widgets::{Block as BlockRatatui, BorderType, Borders, Padding, TitlePosition, Widget as _},
};

#[derive(Debug)]
pub struct Block {
    pub title: Option<(Line, TitlePosition)>,
    pub borders: Borders,
    pub border_style: Style,
    pub border_set: border::Set<'static>,
    pub padding: Padding,
    style: Style,
}

impl Styled for Block {
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}

impl From<Block> for BlockRatatui<'_> {
    fn from(block: Block) -> Self {
        let br: BlockRatatui<'static> = BlockRatatui::new()
            .borders(block.borders)
            .border_style(block.border_style)
            .border_set(block.border_set)
            .padding(block.padding)
            .style(block.style)
            .merge_borders(Block::MERGE_STRATEGY);
        if let Some((title, title_position)) = block.title {
            br.title(title).title_position(title_position)
        } else {
            br
        }
    }
}
impl From<&Block> for BlockRatatui<'_> {
    fn from(block: &Block) -> Self {
        let br: BlockRatatui<'static> = BlockRatatui::new()
            .borders(block.borders)
            .border_style(block.border_style)
            .border_set(block.border_set)
            .padding(block.padding)
            .style(block.style)
            .merge_borders(Block::MERGE_STRATEGY);
        if let &Some((ref title, title_position)) = &block.title {
            br.title(title).title_position(title_position)
        } else {
            br
        }
    }
}

impl Widget for Block {
    async fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf).await;
    }
}

impl WidgetRef for Block {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move { BlockRatatui::from(self).render(area, buf) })
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub const MERGE_STRATEGY: MergeStrategy = MergeStrategy::Replace;
    #[must_use]
    pub const fn new() -> Self {
        Self {
            title: None,
            borders: Borders::ALL,
            border_style: Style::new(),
            border_set: BorderType::Rounded.to_border_set(),
            padding: Padding::proportional(1),
            style: Style::new(),
        }
    }
    #[must_use]
    pub fn inner(&self, area: Rect) -> Rect {
        BlockRatatui::from(self).inner(area)
    }
}
