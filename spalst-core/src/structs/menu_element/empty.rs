//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::MenuEvent, traits::MenuElement};
use core::{any::Any, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use spalstatui::{
    structs::Block,
    traits::{Styled, WidgetRef},
};

/// A completely empty `MenuElement`.
///
/// todo: maybe make this selectable?
/// todo: maybe make this clear the area its displayed in? or a new `MenuElement` for that? or
/// nothing at all.
/// todo: maybe remove `self.block`? After all, this is meant to be a completely empty
/// `MenuElement`; a placeholder.
#[derive(Debug, Default)]
pub struct Empty {
    /// The `Block`.
    block: Option<Block>,
}

impl MenuElement for Empty {
    fn selectable(&self) -> bool {
        false
    }
    fn selected(&self) -> bool {
        false
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
        Box::pin(async move { MenuEvent::Nothing })
    }
}

impl WidgetRef for Empty {
    fn render_ref<'future>(
        &'future self,
        _area: Rect,
        _buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {})
    }
}

impl Styled for Empty {
    fn style(&self) -> Style {
        Style::new()
    }
    fn set_style(&mut self, _style: Style) {}
}
