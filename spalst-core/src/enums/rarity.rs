//! SPDX-License-Identifier: GPL-3.0-only
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}
