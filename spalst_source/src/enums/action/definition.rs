//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::action::{DisplayManagerAction, UpdateAction};
use strum::EnumIs;

/// An action to do.
///
/// Passed from `MenuThing::handle_event()` and executed by `App::handle_action()`.
#[derive(Debug, EnumIs)]
pub enum Action {
    /// To be handled by `DisplayManager`, not `App`.
    DisplayManagerAction(DisplayManagerAction),
    /// Related to updating.
    UpdateAction(UpdateAction),
}
