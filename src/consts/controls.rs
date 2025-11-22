//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::crossterm::event::KeyCode;

// Directions
pub const LEFT_KEYS: [KeyCode; 3] = [KeyCode::Left, KeyCode::Char('a'), KeyCode::Char('h')];
pub const DOWN_KEYS: [KeyCode; 3] = [KeyCode::Down, KeyCode::Char('s'), KeyCode::Char('j')];
pub const UP_KEYS: [KeyCode; 3] = [KeyCode::Up, KeyCode::Char('w'), KeyCode::Char('k')];
pub const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::Right, KeyCode::Char('d'), KeyCode::Char('l')]; // After modifying this, don't forget to modify the ENTER_KEYS const.

// Faster directions
pub const MULTIPLE_DOWN_KEYS: [KeyCode; 1] = [KeyCode::PageDown];
pub const MULTIPLE_UP_KEYS: [KeyCode; 1] = [KeyCode::PageUp];

// Other
pub const ENTER_KEYS: [KeyCode; 4] = [
    KeyCode::Up,
    KeyCode::Char('d'),
    KeyCode::Char('l'),
    KeyCode::Enter,
]; // Mutate this according to the RIGHT_KEYS const.
pub const ESCAPE_KEYS: [KeyCode; 2] = [KeyCode::Esc, KeyCode::Char('q')];
