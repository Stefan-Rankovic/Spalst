//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::MenuEvent, traits::MenuElement};
use core::{any::Any, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect},
    style::Style,
    widgets::{Paragraph, Widget as _},
};
use spalstatui::{
    structs::Block,
    traits::{Styled, WidgetRef},
};

#[derive(Debug)]
pub struct Raw {
    selectable: bool,
    selected: bool,

    block: Option<Block>,

    style: Style,

    pub text: String,
    /// The alignment of the text inside this `MenuElement`.
    text_alignment: HorizontalAlignment,
}

impl Raw {
    pub const fn new(
        selectable: bool,
        selected: bool,
        text: String,
        text_alignment: HorizontalAlignment,
    ) -> Self {
        Self {
            selectable,
            selected,
            block: None,
            style: Style::new(),
            text,
            text_alignment,
        }
    }
    pub const fn styled(
        selectable: bool,
        selected: bool,
        text: String,
        style: Style,
        text_alignment: HorizontalAlignment,
    ) -> Self {
        Self {
            selectable,
            selected,
            block: None,
            style,
            text,
            text_alignment,
        }
    }
}

impl MenuElement for Raw {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected
    }
    fn block(&self) -> Option<&Block> {
        self.block.as_ref()
    }
    fn set_block(&mut self, block: Block) -> Option<Block> {
        self.block.replace(block)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn handle_key_event(
        &self,
        _event: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = MenuEvent> + Send>> {
        Box::pin(async move { todo!() })
    }
}

impl WidgetRef for Raw {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            let mut paragraph: Paragraph<'_> =
                Paragraph::new(self.text.as_str()).alignment(self.text_alignment);
            if let Some(block) = self.block() {
                paragraph = paragraph.block(block.into());
            }
            paragraph.render(area, buf);
        })
    }
}

impl Styled for Raw {
    fn style(&self) -> Style {
        self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
