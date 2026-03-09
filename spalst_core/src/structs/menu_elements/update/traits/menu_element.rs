//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::MenuEvent, structs::menu_elements::MenuElementsUpdate, traits::MenuElement};
use spalstatui::structs::Block;
use std::{any::Any, pin::Pin};

impl MenuElement for MenuElementsUpdate {
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
        key_event: crossterm::event::KeyEvent,
    ) -> Pin<Box<dyn Future<Output = MenuEvent> + Send>> {
        Box::pin(async move { todo!() })
    }
}
