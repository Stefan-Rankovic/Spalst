//! SPDX-License-Identifier: GPL-3.0-only
use crate::enums::MainMenuEnum;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Select {
    Previous,
    Next,
    Direct(MainMenuEnum),
}
