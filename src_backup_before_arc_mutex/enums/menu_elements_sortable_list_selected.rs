//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::{MenuElementRaw, MenuElementsSortableList},
    traits::{EnumAsStr, MenuElement, MenuElementsSelectedEnum},
};
use std::fmt::Debug;
use strum::{IntoDiscriminant, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumDiscriminants, EnumIs, EnumIter};

#[derive(Clone, Copy, Debug, EnumCountMacro, EnumDiscriminants, EnumIs, Eq, PartialEq)]
#[strum_discriminants(derive(EnumIter))]
pub enum MenuElementsSortableListSelected<ItemId: Copy + Debug> {
    SortMethod,
    SortAscending,
    Items(ItemId),
}

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
    fn as_str_user(&self) -> &str {
        match self.discriminant() {
            MenuElementsSortableListSelectedDiscriminants::SortMethod => "Self::SortMethod",
            MenuElementsSortableListSelectedDiscriminants::SortAscending => "Self::SortAscending",
            MenuElementsSortableListSelectedDiscriminants::Items => "Self::Items",
        }
    }

    fn as_str_debug(&self) -> &str {
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
        if self.is_sort_ascending() {
            Some(Self::Discriminant::SortMethod)
        } else {
            None
        }
    }

    fn select_down(&self) -> Option<Self::Discriminant> {
        if self.is_items() {
            None
        } else {
            Some(Self::Discriminant::Items)
        }
    }

    fn select_up(&self) -> Option<Self::Discriminant> {
        if self.is_items() {
            Some(Self::Discriminant::SortMethod)
        } else {
            None
        }
    }

    fn select_right(&self) -> Option<Self::Discriminant> {
        if self.is_sort_method() {
            Some(Self::Discriminant::SortAscending)
        } else {
            None
        }
    }
}
