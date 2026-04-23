//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{Renderable, Styled};
use core::mem;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::{border, merge::MergeStrategy},
    text::Line,
    widgets::{Block as BlockRatatui, BorderType, Borders, Padding, TitlePosition, Widget as _},
};

#[derive(Debug)]
pub struct BlockDisplay {
    pub title: Option<(Line<'static>, TitlePosition)>,
    pub borders: Borders,
    pub border_style: Style,
    pub border_set: border::Set<'static>,
    pub padding: Padding,
    style: Style,
}

impl Styled for BlockDisplay {
    fn get_style(&self) -> Style {
        self.style
    }

    fn set_style(
        &mut self,
        new_style: Style,
    ) -> Style {
        mem::replace(&mut self.style, new_style)
    }
}

impl From<BlockDisplay> for BlockRatatui<'static> {
    fn from(block_display: BlockDisplay) -> Self {
        Self::from(&block_display)
    }
}
impl From<&BlockDisplay> for BlockRatatui<'static> {
    fn from(block: &BlockDisplay) -> Self {
        let br: BlockRatatui<'static> = BlockRatatui::new()
            .borders(block.borders)
            .border_style(block.border_style)
            .border_set(block.border_set)
            .padding(block.padding)
            .style(block.style)
            .merge_borders(BlockDisplay::MERGE_STRATEGY);
        if let &Some((ref title, title_position)) = &block.title {
            br.title(title.clone()).title_position(title_position)
        } else {
            br
        }
    }
}

impl Renderable for BlockDisplay {
    fn basic_render(
        &self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        BlockRatatui::from(self).render(area, buf);
    }
}

impl BlockDisplay {
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
    pub fn inner(
        &self,
        area: Rect,
    ) -> Rect {
        BlockRatatui::from(self).inner(area)
    }
}
