//! SPDX-License-Identifier: GPL-3.0-only
//! todo: make all of the modules inside not end with _mod

mod convert_path_mod;
mod fetch_unsafe_versions;
// mod display; // todo: this was removed temporarily to see if it can be removed permanently. if the program works and displays fine without this, it's safe to remove.
mod display_duration_mod;
mod epr_dbg_mod;
mod epr_wrn_mod;
mod keycode_to_string_mod;
mod keycodes_to_string_mod;
mod set_up_logging_mod;
mod style;

pub use convert_path_mod::convert_path;
pub use epr_dbg_mod::epr_dbg;
pub use epr_wrn_mod::epr_wrn;
// pub use display::{create_block, create_popup, create_popup_area, render_items_in_area}; // todo: uncomment maybe
pub use display_duration_mod::{display_duration, display_duration_ago_format};
pub use fetch_unsafe_versions::fetch_unsafe_versions;
pub use keycode_to_string_mod::keycode_to_string;
pub use keycodes_to_string_mod::keycodes_to_string;
pub use set_up_logging_mod::set_up_logging;
