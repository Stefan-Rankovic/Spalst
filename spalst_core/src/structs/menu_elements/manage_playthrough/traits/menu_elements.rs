//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuElementsManagePlaythroughSelected,
    structs::MenuElementsManagePlaythrough,
    traits::{MenuElement, MenuElements},
};
use ratatui::layout::Rect;

impl MenuElements for MenuElementsManagePlaythrough {
    type Elements = MenuElementsManagePlaythroughSelected;

    fn selected_element(&self) -> Option<&Self::Elements> {
        self.selected.as_ref()
    }
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements> {
        self.selected.as_mut()
    }

    async fn elements(&self) -> Vec<Box<dyn MenuElement>> {
        todo!()
    }

    fn elements_area(&self, _available_area: Rect) -> Vec<Rect> {
        todo!()
    }
}
