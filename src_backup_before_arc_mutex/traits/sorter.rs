//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::SortMethod as SortMethodTrait;

pub trait Sorter<SortMethod: SortMethodTrait>: Copy {
    type Item;

    fn sort_method(&self) -> &SortMethod;
    fn sort_ascending(&self) -> &bool;

    fn sort_items<'items>(&self, items: &'items [Self::Item]) -> Vec<&'items Self::Item>;
}
