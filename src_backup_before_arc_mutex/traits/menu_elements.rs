//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{MenuElement, MenuElementsSelectedEnum};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::fmt::Debug;

/// Defines a struct menu.
pub trait MenuElements: Debug + MenuElement + Widget {
    type Elements: MenuElementsSelectedEnum;

    /// Returns the currently selected element if there is one, otherwise None.
    fn selected_element(&self) -> Option<&Self::Elements>;
    /// Returns the currently selected element as mutable if there is one, otherwise None.
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements>;

    /// Returns all child elements.
    fn elements(&self) -> &[Box<dyn MenuElement>];
    /// Returns the areas of all child elements.
    fn elements_area(&self) -> &[Rect];

    fn render_elements(&self, area: Rect, buf: &mut Buffer) {
        for (element, element_area) in self.elements().iter().zip(self.elements_area().iter()) {
            element.render(element_area, buf);
        }
    }
}
