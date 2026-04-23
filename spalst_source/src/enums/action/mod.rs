//! SPDX-License-Identifier: GPL-3.0-only

mod definition;
mod display_manager_action;
mod update_action;

pub use definition::Action;
#[expect(
    clippy::module_name_repetitions,
    reason = "`allow-exact-repetitions` should already allow this, but it doesn't for some reason"
)]
pub use display_manager_action::DisplayManagerAction;
#[expect(
    clippy::module_name_repetitions,
    reason = "`allow-exact-repetitions` should already allow this, but it doesn't for some reason"
)]
pub use update_action::UpdateAction;
