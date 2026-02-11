//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{DOWN_KEYS, LEFT_KEYS, MULTIPLE_DOWN_KEYS, MULTIPLE_UP_KEYS, RIGHT_KEYS, UP_KEYS},
    enums::MenuEvent,
    structs::{MenuElementsMainMenu, SelectAmount},
    traits::MenuElement,
};
use core::{any::Any, pin::Pin};
use crossterm::event::KeyEvent;
use spalstatui::structs::Block;

impl MenuElement for MenuElementsMainMenu {
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
        key_event: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = MenuEvent> + Send>> {
        Box::pin(async move {
            match key_event.code {
                key_code if LEFT_KEYS.contains(&key_code) => {
                    MenuEvent::SelectLeft(SelectAmount::ONE)
                }
                key_code if DOWN_KEYS.contains(&key_code) => {
                    MenuEvent::SelectDown(SelectAmount::ONE)
                }
                key_code if MULTIPLE_DOWN_KEYS.contains(&key_code) => {
                    MenuEvent::SelectDown(SelectAmount::MULTIPLE)
                }
                key_code if UP_KEYS.contains(&key_code) => MenuEvent::SelectUp(SelectAmount::ONE),
                key_code if MULTIPLE_UP_KEYS.contains(&key_code) => {
                    MenuEvent::SelectUp(SelectAmount::MULTIPLE)
                }
                key_code if RIGHT_KEYS.contains(&key_code) => {
                    MenuEvent::SelectRight(SelectAmount::ONE)
                }
                _ => todo!(),
            }
        })
    }
}
