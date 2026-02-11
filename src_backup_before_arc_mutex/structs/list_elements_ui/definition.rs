//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::ListElementsUiEnum,
    traits::{AsDisplayable, Sorter as SorterTrait, sort_method::SortMethod as SortMethodTrait},
};
use std::collections::HashMap;

#[deprecated = "Use MenuElementList or MenuElementsSortableList instead."]
#[derive(Debug)]
pub struct ListElementsUi<
    'h,
    't,
    ElementType: Copy + Eq,
    DataElementType: AsDisplayable,
    SortMethod: SortMethodTrait,
    Sorter: SorterTrait<SortMethod>,
> {
    pub selected: Option<ListElementsUiEnum<ElementType>>,
    pub elements: &'h HashMap<ElementType, DataElementType>,
    pub sorter: Sorter,
    pub title: Option<&'t str>,
    pub display_sort_options: bool,
    pub display_block: bool,
    pub display_note: bool,
    pub element_spacing: u16,
}
