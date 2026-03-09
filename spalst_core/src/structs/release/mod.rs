//! SPDX-License-Identifier: GPL-3.0-only

mod definition;
mod display_notes;
#[cfg(target_family = "unix")]
mod unix_update;
mod updating;
#[cfg(target_family = "windows")]
mod windows_update;

pub use definition::Release;
