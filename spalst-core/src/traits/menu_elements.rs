//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{MenuElement, MenuElementsSelectedEnum};
use core::fmt::Debug;
use ratatui::layout::Rect;
use strum::{IntoDiscriminant, IntoEnumIterator};

/// Defines a struct menu.
pub trait MenuElements: Debug + MenuElement
where
    <Self::Elements as IntoDiscriminant>::Discriminant: IntoEnumIterator,
{
    type Elements: MenuElementsSelectedEnum;

    /// Returns the currently selected element if there is one, otherwise None.
    fn selected_element(&self) -> Option<&Self::Elements>;
    /// Returns the currently selected element as mutable if there is one, otherwise None.
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements>;

    /// Returns all child elements.
    fn elements(&self) -> impl Future<Output = Vec<Box<dyn MenuElement>>>;
    /// Returns the areas of all child elements.
    fn elements_area(&self, available_area: Rect) -> Vec<Rect>;
}
