//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{MenuEvent, MessageWhenEmptyList},
    traits::{AsDisplayable, MenuElement},
};
use core::{any::Any, fmt::Debug, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::style::Style;
use spalstatui::structs::Block;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct List<
    ItemId: Copy + Debug + PartialEq + Send + Sync,
    Item: AsDisplayable + Debug + PartialEq + Send + Sync,
> {
    selectable: bool,
    pub selected: Option<ItemId>,

    block: Option<Block>,

    pub(in crate::structs::menu_element::list) style: Style,

    pub items: Arc<Mutex<[(ItemId, Item)]>>,

    pub message_when_empty: MessageWhenEmptyList,
    pub show_elements_counter: bool,
    pub item_spacing: u16,
}

impl<
    ItemId: Copy + Debug + PartialEq + Send + Sync,
    Item: AsDisplayable + Debug + PartialEq + Send + Sync,
> List<ItemId, Item>
{
    pub const fn new(
        selectable: bool,
        selected: Option<ItemId>,
        items: Arc<Mutex<[(ItemId, Item)]>>,
        message_when_empty: MessageWhenEmptyList,
        show_elements_counter: bool,
        item_spacing: u16,
    ) -> Self {
        Self {
            selectable,
            selected,
            items,
            block: None,
            style: Style::new(),
            message_when_empty,
            show_elements_counter,
            item_spacing,
        }
    }
    pub const fn styled(
        selectable: bool,
        selected: Option<ItemId>,
        items: Arc<Mutex<[(ItemId, Item)]>>,
        style: Style,
        message_when_empty: MessageWhenEmptyList,
        show_elements_counter: bool,
        item_spacing: u16,
    ) -> Self {
        Self {
            selectable,
            selected,
            items,
            block: None,
            style,
            message_when_empty,
            show_elements_counter,
            item_spacing,
        }
    }
}

impl<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> MenuElement for List<ItemId, Item>
{
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
