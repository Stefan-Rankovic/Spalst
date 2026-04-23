//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::MenuThing;

/// Returned after a `MenuThing` handles an event.
#[derive(Debug)]
pub enum DisplayManagerAction {
    /// Add a `MenuThing` to the front of the history.
    ///
    /// See `SelectNewMT` in order to select it as well.
    NewMT(Box<dyn MenuThing>),
    /// Add a `MenuThing` to the front of the history and select it.
    SelectNewMT(Box<dyn MenuThing>),
    /// Select the previous `MenuThing`, if one. If not, does nothing.
    SelectPrevMT,
    /// Select the next `MenuThing`, if one. If not, does nothing.
    SelectNextMT,
    // todo: add more discriminants here to handle all the methods on `DisplayManager`.
}
