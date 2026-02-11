//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuElementsSortableListSelected,
    structs::{MenuElementList, MenuElementRaw, MenuElementsSortableList},
    traits::{AsDisplayable, MenuElement, MenuElements},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::{fmt::Debug, rc::Rc};
use strum::{IntoDiscriminant, IntoEnumIterator};

impl<'data, ItemId: Copy + Debug + Eq, Item: AsDisplayable + Debug> MenuElements
    for MenuElementsSortableList<'data, ItemId, Item>
{
    type Elements = MenuElementsSortableListSelected<ItemId>;

    fn selected_element(&mut self) -> &mut Self::Elements {
        &mut self.selected_element
    }

    fn elements(&self) -> Vec<Box<dyn MenuElement>> {
        <Self::Elements as IntoDiscriminant>::Discriminant::iter()
            .map(|variant: <Self::Elements as IntoDiscriminant>::Discriminant| -> Box<dyn MenuElement> {
                Box::new(match variant {
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortMethod => MenuElementRaw::new(
                        true,
                        self.selected_element == Self::Elements::SortMethod,
                        self.sort_method,
                    ),
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortAscending => MenuElementRaw::new(
                        true,
                        self.selected_element == Self::Elements::SortAscending,
                        self.sort_ascending,
                    ),
                    <Self::Elements as IntoDiscriminant>::Discriminant::Items => MenuElementList::new(
                        true,
                        if self.selected_element.discriminant() == <Self::Elements as IntoDiscriminant>::Discriminant::Items {self.items.get(0)} else {None},
                        self.items,
                        self.items.get(0),
                        true,
                        1,
                    ),
                })
            })
            .collect()
    }

    fn elements_area(&self) -> &[Rect] {
        let (sort_options_area, items_area): (Rect, Rect) = {
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1 + 2), Constraint::Min(1)]);
            (parts[0], parts[1])
        };

        // todo: figure out whether there's a better place for this.
        let horizontal_padding: usize = 1;
        let (sort_method_area, sort_ascending_area): (Rect, Rect) = {
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(
                        self.sort_method.chars().count().try_into().unwrap()
                            + horizontal_padding * 2 // Horizontal padding on each side
                            + 2, // Block borders
                    ),
                    Constraint::Min(0),
                    Constraint::Length(
                        self.sort_ascending.chars().count().try_into().unwrap()
                            + horizontal_padding * 2 // Horizontal padding on each side
                            + 2, // Block borders
                    ),
                ]);
            (parts[0], parts[2])
        };

        Self::Elements::iter().map(|variant: Self::Elements| -> Rect {
            match variant {
                Self::Elements::SortMethod => sort_method_area,
                Self::Elements::SortAscending => sort_ascending_area,
                Self::Elements::Items(item_id) => items_area,
            }
        })
    }
}
