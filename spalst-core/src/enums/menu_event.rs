//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::SelectAmount;

/// Represents an event that should be done.
#[derive(Debug)]
pub enum MenuEvent {
    /// The event is to be ignored.
    Nothing,

    SelectLeft(SelectAmount),
    SelectDown(SelectAmount),
    SelectUp(SelectAmount),
    SelectRight(SelectAmount),
}
