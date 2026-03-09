//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{MenuElementsSortableListSelected, MessageWhenEmptyList},
    structs::{MenuElementList, MenuElementRaw, MenuElementsSortableList},
    traits::{AsDisplayable, MenuElement, MenuElements},
};
use core::fmt::Debug;
use futures::future::join_all;
use ratatui::layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect};
use std::{rc::Rc, sync::Arc};
use strum::{IntoDiscriminant, IntoEnumIterator as _};

impl<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> MenuElements for MenuElementsSortableList<ItemId, Item>
{
    type Elements = MenuElementsSortableListSelected<ItemId>;

    fn selected_element(&self) -> Option<&Self::Elements> {
        self.selected.as_ref()
    }
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements> {
        self.selected.as_mut()
    }

    async fn elements(&self) -> Vec<Box<dyn MenuElement>> {
        join_all(
            <Self::Elements as IntoDiscriminant>::Discriminant::iter().map(async |variant| {
                #[expect(
                    clippy::as_conversions,
                    reason = "The only way to make a MenuElement struct be a dyn MenuElement."
                )]
                match variant {
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortMethod => {
                        Box::new(MenuElementRaw::new(
                            true,
                            self.selected_element() == Some(&Self::Elements::SortMethod),
                            self.sort_method.to_string(),
                            HorizontalAlignment::Center,
                        )) as Box<dyn MenuElement>
                    }
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortAscending => {
                        Box::new(MenuElementRaw::new(
                            true,
                            self.selected_element() == Some(&Self::Elements::SortAscending),
                            self.sort_ascending.to_string(),
                            HorizontalAlignment::Center,
                        )) as Box<dyn MenuElement>
                    }
                    <Self::Elements as IntoDiscriminant>::Discriminant::Items => {
                        Box::new(MenuElementList::new(
                            true,
                            if let Some(selected_item) = self.selected_element()
                                && selected_item.discriminant()
                                    == <Self::Elements as IntoDiscriminant>::Discriminant::Items
                            {
                                self.items.lock().await.first().map(|&(id, _)| id)
                            } else {
                                None
                            },
                            Arc::clone(&self.items),
                            MessageWhenEmptyList::Default,
                            true,
                            1,
                        )) as Box<dyn MenuElement>
                    }
                }
            }),
        )
        .await
    }

    fn elements_area(&self, available_area: Rect) -> Vec<Rect> {
        let (sort_options_area, items_area): (Rect, Rect) = {
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1 + 2), Constraint::Min(1)])
                .split(available_area);
            (parts[0], parts[1])
        };

        // todo: figure out whether there's a better place for this.
        let horizontal_padding: usize = 1;
        let (sort_method_area, sort_ascending_area): (Rect, Rect) = {
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(
                        (
                            // Block borders so + 2
                            // Horizontal padding on each side so * 2
                            self.sort_method.chars().count() + horizontal_padding * 2 + 2
                        )
                        .try_into()
                        .unwrap(),
                    ),
                    Constraint::Min(0),
                    Constraint::Length(
                        (
                            // Block borders so + 2
                            // Horizontal padding on each side so * 2
                            self.sort_ascending.chars().count() + horizontal_padding * 2 + 2
                        )
                        .try_into()
                        .unwrap(),
                    ),
                ])
                .split(sort_options_area);
            (parts[0], parts[2])
        };

        <Self::Elements as IntoDiscriminant>::Discriminant::iter()
            .map(|variant| -> Rect {
                match variant {
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortMethod => {
                        sort_method_area
                    }
                    <Self::Elements as IntoDiscriminant>::Discriminant::SortAscending => {
                        sort_ascending_area
                    }
                    <Self::Elements as IntoDiscriminant>::Discriminant::Items => items_area,
                }
            })
            .collect()
    }
}
