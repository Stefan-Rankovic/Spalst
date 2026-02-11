//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::MenuEvent, traits::MenuElement};
use core::{any::Any, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget as _};
use spalstatui::{
    structs::Block,
    traits::{Styled, WidgetRef},
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct KeyValue {
    selectable: bool,
    selected: bool,

    block: Option<Block>,

    style: Style,

    key: Arc<RwLock<String>>,
    value: Arc<RwLock<String>>,
}

impl MenuElement for KeyValue {
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

impl WidgetRef for KeyValue {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            format!("{}: {}", self.key.read().await, self.value.read().await)
                .as_str()
                .render(area, buf);
        })
    }
}

impl Styled for KeyValue {
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
