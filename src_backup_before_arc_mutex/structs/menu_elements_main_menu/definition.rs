//! SPDX-License-Identifier: GPL-3.0-only

use std::any::Any;

use crate::{
    enums::MenuElementsMainMenuSelected,
    traits::{MenuElement, MenuElements},
};

#[derive(Clone, Copy, Debug)]
pub struct MenuElementsMainMenu {
    selectable: bool,
    pub selected: Option<<Self as MenuElements>::Elements>,
}

impl MenuElementsMainMenu {
    pub fn new(selectable: bool, selected: Option<<Self as MenuElements>::Elements>) -> Self {
        Self {
            selectable,
            selected,
        }
    }
}

impl MenuElement for MenuElementsMainMenu {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected.is_some()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
