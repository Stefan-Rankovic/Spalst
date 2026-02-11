//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::{EnumAsStr, MenuElementsSelectedEnum};
use core::fmt::Debug;
use strum::IntoDiscriminant as _;
use strum_macros::{EnumCount as EnumCountMacro, EnumDiscriminants, EnumIs, EnumIter};

#[derive(Clone, Copy, Debug, EnumCountMacro, EnumDiscriminants, EnumIs, Eq, PartialEq)]
#[strum_discriminants(derive(EnumIter))]
pub enum MenuElementsSortableListSelected<ItemId: Copy + Debug> {
    SortMethod,
    SortAscending,
    Items(ItemId),
}

// todo: maybe this is needed?
// impl<ItemId: Copy + Debug> MenuElementsSortableListSelected<ItemId> {
//     pub const fn elements() {
//         Self::iter().map(|variant| match variant {
//             Self::SortMethod => MenuElementRaw,
//             Self::SortAscending => MenuElementRaw,
//             Self::Items(_) => MenuElementsSortableList,
//         })
//     }
// }

impl<ItemId: Copy + Debug> EnumAsStr for MenuElementsSortableListSelected<ItemId> {
    fn as_str_user(&self) -> &'static str {
        unreachable!()
    }

    fn as_str_debug(&self) -> &'static str {
        match self.discriminant() {
            MenuElementsSortableListSelectedDiscriminants::SortMethod => {
                "MenuElementsSortableListSelected::SortMethod"
            }
            MenuElementsSortableListSelectedDiscriminants::SortAscending => {
                "MenuElementsSortableListSelected::SortAscending"
            }
            MenuElementsSortableListSelectedDiscriminants::Items => {
                "MenuElementsSortableListSelected::Items"
            }
        }
    }
}

impl<ItemId: Copy + Debug> MenuElementsSelectedEnum for MenuElementsSortableListSelected<ItemId> {
    fn select_left(&self) -> Option<Self::Discriminant> {
        self.is_sort_ascending()
            .then_some(Self::Discriminant::SortMethod)
    }

    fn select_down(&self) -> Option<Self::Discriminant> {
        if self.is_items() {
            None
        } else {
            Some(Self::Discriminant::Items)
        }
    }

    fn select_up(&self) -> Option<Self::Discriminant> {
        self.is_items().then_some(Self::Discriminant::SortMethod)
    }

    fn select_right(&self) -> Option<Self::Discriminant> {
        self.is_sort_method()
            .then_some(Self::Discriminant::SortAscending)
    }
}
