//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{ManagePlaythroughsSelected as Selected, PlaythroughsSortBy as SortBy},
    structs::{Playthrough, PlaythroughName, Playthroughs},
};
use strum_macros::EnumDiscriminants;

#[derive(Debug, EnumDiscriminants)]
pub enum ManagePlaythroughsMenu {
    Select {
        selected: Selected,
        sort_ascending: bool,
        sort_by: SortBy,
    },
    Playthrough(PlaythroughName),
}

impl Default for ManagePlaythroughsMenu {
    fn default() -> Self {
        Self::build_select()
    }
}

impl ManagePlaythroughsMenu {
    pub fn build_select() -> Self {
        Self::Select {
            selected: Selected::default(),
            sort_ascending: false,
            sort_by: SortBy::default(),
        }
    }
}
