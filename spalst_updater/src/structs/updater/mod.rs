//! SPDX-License-Identifier: GPL-3.0-only

mod definition;
mod exit_early;
mod running;
mod should_program_quit;
mod start_updating;
mod try_new;
mod update_to_latest;
mod update_to_release;
mod update_to_safe;

pub use definition::Updater;
