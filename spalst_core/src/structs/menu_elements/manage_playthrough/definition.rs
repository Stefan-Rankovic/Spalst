//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuEvent,
    traits::{MenuElement, MenuElements},
};
use core::{any::Any, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::style::Style;
use spalstatui::{structs::Block, traits::Styled};

#[derive(Debug)]
pub struct ManagePlaythrough {
    selectable: bool,
    pub selected: Option<<Self as MenuElements>::Elements>,

    block: Option<Block>,

    style: Style,
}

impl MenuElement for ManagePlaythrough {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected.is_some()
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

impl Styled for ManagePlaythrough {
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
