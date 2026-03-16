//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::SelectAmount;

#[derive(Debug)]
pub enum MenuEvent {
    Nothing,

    SelectLeft(SelectAmount),
    SelectDown(SelectAmount),
    SelectUp(SelectAmount),
    SelectRight(SelectAmount),
}
