//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::keycode_to_string;
use ratatui::crossterm::event::KeyCode;

pub fn keycodes_to_string(kcs: &[KeyCode]) -> String {
    kcs.iter()
        .map(keycode_to_string)
        .collect::<Vec<String>>()
        .join(", ")
}
