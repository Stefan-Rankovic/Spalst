//! SPDX-License-Identifier: GPL-3.0-only

use std::fmt::Debug;

use strum_macros::EnumIs;

#[derive(Clone, Copy, Debug, EnumIs, Eq, PartialEq)]
pub enum MenuElementListSelected<ItemId: Copy + Debug> {
    SortMethod,
    SortAscending,
    Items(ItemId),
}
