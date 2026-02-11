//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{AsDisplayable, MenuElement};
use std::fmt::Debug;

#[derive(Debug)]
pub struct MenuElementList<'data, ItemId: Copy + Debug, Item: AsDisplayable + Debug> {
    selectable: bool,
    selected: Option<ItemId>,

    pub items: &'data [(ItemId, Item)],

    pub default_selected_item: Option<ItemId>,
    pub counter: bool,
    pub item_spacing: u16,
}

impl<'data, ItemId: Copy + Debug, Item: AsDisplayable + Debug>
    MenuElementList<'data, ItemId, Item>
{
    pub fn new(
        selectable: bool,
        selected: Option<ItemId>,
        items: &'data [(ItemId, Item)],
        default_selected_item: Option<ItemId>,
        counter: bool,
        item_spacing: u16,
    ) -> Self {
        Self {
            selectable,
            selected,
            items,
            default_selected_item,
            counter,
            item_spacing,
        }
    }
}

impl<ItemId: Copy + Debug, Item: AsDisplayable + Debug> Clone
    for MenuElementList<'_, ItemId, Item>
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<ItemId: Copy + Debug, Item: AsDisplayable + Debug> Copy for MenuElementList<'_, ItemId, Item> {}

impl<'data, ItemId: Copy + Debug, Item: AsDisplayable + Debug> MenuElement
    for MenuElementList<'data, ItemId, Item>
{
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected.is_some()
    }
}
