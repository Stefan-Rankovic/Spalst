//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{ManagePlaythroughSelectedEnum, PlaythroughsSortMethod},
    structs::{
        ListElementsUi, ManagePlaythroughMenu, Playthrough, PlaythroughId, PlaythroughsSorter,
    },
};
use std::collections::HashMap;
use strum_macros::EnumDiscriminants;

#[derive(Debug, EnumDiscriminants)]
pub enum ManagePlaythroughsMenu<'h, 'n> {
    Select(
        ListElementsUi<
            'h,
            'static,
            PlaythroughId,
            Playthrough<'n>,
            PlaythroughsSortMethod,
            PlaythroughsSorter,
        >,
    ),
    Playthrough(ManagePlaythroughMenu<'h, 'n>),
}

impl<'h, 'n> ManagePlaythroughsMenu<'h, 'n> {
    pub fn build_select(elements: &'h HashMap<PlaythroughId, Playthrough<'n>>) -> Self {
        Self::Select(ListElementsUi {
            selected: None,
            elements,
            sorter: PlaythroughsSorter::new(PlaythroughsSortMethod::LastPlayedAt, true),
            title: Some("Select playthrough"),
            display_sort_options: true,
            display_block: true,
            display_note: true,
            element_spacing: 1,
        })
    }
}
