//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{AsDisplayable, MenuElements};
use core::fmt::Debug;
use ratatui::style::Style;
use spalstatui::structs::Block;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct SortableList<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> {
    pub(in crate::structs::menu_elements::sortable_list) selectable: bool,
    pub(in crate::structs::menu_elements::sortable_list) selected:
        Option<<Self as MenuElements>::Elements>,

    pub(in crate::structs::menu_elements::sortable_list) block: Option<Block>,

    pub(in crate::structs::menu_elements::sortable_list) style: Style,

    pub sort_method: &'static str,
    pub sort_ascending: &'static str,
    pub items: Arc<Mutex<[(ItemId, Item)]>>,
}
