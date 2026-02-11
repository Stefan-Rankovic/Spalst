//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::layout::Rect;

use crate::{
    enums::MenuElementsManagePlaythroughSelected,
    structs::MenuElementsManagePlaythrough,
    traits::{MenuElement, MenuElements},
};

impl MenuElements for MenuElementsManagePlaythrough {
    type Elements = MenuElementsManagePlaythroughSelected;

    fn selected_element(&mut self) -> &mut Self::Elements {
        &mut self.selected_element
    }

    fn elements(&self) -> &[Box<dyn MenuElement>] {
        todo!()
    }

    fn elements_area(&self) -> &[Rect] {
        todo!()
    }
}
