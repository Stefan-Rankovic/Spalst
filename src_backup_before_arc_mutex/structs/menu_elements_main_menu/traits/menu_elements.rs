//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuElementsMainMenuSelected,
    structs::MenuElementsMainMenu,
    traits::{MenuElement, MenuElements},
};

impl MenuElements for MenuElementsMainMenu {
    type Elements = MenuElementsMainMenuSelected;

    fn selected_element(&self) -> Option<&Self::Elements> {
        self.selected.as_ref()
    }
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements> {
        self.selected.as_mut()
    }

    fn elements(&self) -> &[Box<dyn MenuElement>] {
        todo!()
    }

    fn elements_area(&self) -> &[ratatui::prelude::Rect] {
        todo!()
    }
}
