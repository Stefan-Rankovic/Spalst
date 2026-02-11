//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{AsDisplayable, MenuElement, MenuElements};
use std::fmt::Debug;

#[derive(Debug)]
pub struct MenuElementsSortableList<'data, ItemId: Copy + Debug + Eq, Item: AsDisplayable + Debug> {
    pub selectable: bool,
    pub selected: bool,
    pub selected_element: <Self as MenuElements>::Elements,

    pub sort_method: &'static str,
    pub sort_ascending: &'static str,
    pub items: &'data [(ItemId, Item)],
}

impl<ItemId: Copy + Debug + Eq, Item: AsDisplayable + Debug> Clone
    for MenuElementsSortableList<'_, ItemId, Item>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<ItemId: Copy + Debug + Eq, Item: AsDisplayable + Debug> Copy
    for MenuElementsSortableList<'_, ItemId, Item>
{
}

impl<ItemId: Copy + Debug + Eq, Item: AsDisplayable + Debug> MenuElement
    for MenuElementsSortableList<'_, ItemId, Item>
{
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected
    }
}
