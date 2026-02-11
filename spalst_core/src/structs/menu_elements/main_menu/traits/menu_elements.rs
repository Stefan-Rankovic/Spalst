//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuElementsMainMenuSelected,
    structs::{MenuElementRaw, MenuElementsMainMenu},
    traits::{EnumAsStr as _, MenuElement, MenuElements},
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use spalstatui::{structs::Block, traits::Styled as _};
use std::rc::Rc;
use strum::{EnumCount, IntoDiscriminant, IntoEnumIterator as _};

impl MenuElements for MenuElementsMainMenu {
    type Elements = MenuElementsMainMenuSelected;

    fn selected_element(&self) -> Option<&Self::Elements> {
        self.selected.as_ref()
    }
    fn selected_element_mut(&mut self) -> Option<&mut Self::Elements> {
        self.selected.as_mut()
    }

    async fn elements(&self) -> Vec<Box<dyn MenuElement>> {
        <Self::Elements as IntoDiscriminant>::Discriminant::iter().map(
            |variant: <Self::Elements as IntoDiscriminant>::Discriminant| -> Box<dyn MenuElement> {
                let text: String = variant.as_str_user().to_string();
                let selected: bool = self.selected_element().map(IntoDiscriminant::discriminant) == Some(variant);
                Box::new(match variant {
                    <Self::Elements as IntoDiscriminant>::Discriminant::Continue => {
                        let mut element: MenuElementRaw = MenuElementRaw::new(true, selected, text);
                        if !self.last_played_available {
                            element.italic();
                            element.dim();
                        }
                        element
                    }
                    < Self::Elements as IntoDiscriminant>::Discriminant::CreatePlaythrough | <Self::Elements as IntoDiscriminant>::Discriminant::ManagePlaythroughs | <Self::Elements as IntoDiscriminant>::Discriminant::Settings|<Self::Elements as IntoDiscriminant>::Discriminant::Achievements|<Self::Elements as IntoDiscriminant>::Discriminant::Quit=> {
                        MenuElementRaw::new(true, selected, text)
                    }
                }.with_block(Block::new()).0)
            },
        ).collect()
    }

    fn elements_area(&self, available_area: Rect) -> Vec<Rect> {
        const ITEM_HEIGHT: u16 = 3;
        // Create centered layout
        let vertical_chunks: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(available_area);
        let horizontal_chunks: Rc<[Rect]> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(vertical_chunks[1]);
        let menu_area: Rect = horizontal_chunks[1];
        // Calculate spacing for menu items
        let menu_items_area: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(ITEM_HEIGHT); <Self::Elements as EnumCount>::COUNT])
            .split(menu_area);
        <Self::Elements as IntoDiscriminant>::Discriminant::iter()
            .map(
                |variant: <Self::Elements as IntoDiscriminant>::Discriminant| -> Rect {
                    match variant {
                        <Self::Elements as IntoDiscriminant>::Discriminant::Continue => {
                            menu_items_area[0]
                        }
                        <Self::Elements as IntoDiscriminant>::Discriminant::CreatePlaythrough => {
                            menu_items_area[1]
                        }
                        <Self::Elements as IntoDiscriminant>::Discriminant::ManagePlaythroughs => {
                            menu_items_area[2]
                        }
                        <Self::Elements as IntoDiscriminant>::Discriminant::Achievements => {
                            menu_items_area[3]
                        }
                        <Self::Elements as IntoDiscriminant>::Discriminant::Settings => {
                            menu_items_area[4]
                        }
                        <Self::Elements as IntoDiscriminant>::Discriminant::Quit => {
                            menu_items_area[5]
                        }
                    }
                },
            )
            .collect()
    }
}
