//! SPDX-License-Identifier: GPL-3.0-only

// todo: maybe remove some of these
// mod exit_early;
// mod prompt_and_wait;
// mod prompt_input;
// mod running;
// mod should_program_quit;
// mod start_updating;
mod available_releases;
mod can_update;
mod definition;
mod update_to_latest;
mod update_to_release;

pub use definition::SafeUpdater;
