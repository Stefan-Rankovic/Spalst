//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::crossterm::event::KeyCode;

// Directions
/// Keys that, when pressed, act as a left key.
pub const LEFT_KEYS: [KeyCode; 3] = [KeyCode::Left, KeyCode::Char('a'), KeyCode::Char('h')];
/// Keys that, when pressed, act as a down key.
pub const DOWN_KEYS: [KeyCode; 3] = [KeyCode::Down, KeyCode::Char('s'), KeyCode::Char('j')];
/// Keys that, when pressed, act as a up key.
pub const UP_KEYS: [KeyCode; 3] = [KeyCode::Up, KeyCode::Char('w'), KeyCode::Char('k')];
/// Keys that, when pressed, act as a right key.
pub const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::Right, KeyCode::Char('d'), KeyCode::Char('l')]; // After modifying this, don't forget to modify the ENTER_KEYS const.

// Faster directions
/// Keys that, when pressed, act as multiple down keys.
pub const MULTIPLE_DOWN_KEYS: [KeyCode; 1] = [KeyCode::PageDown];
/// Keys that, when pressed, act as multiple up keys.
pub const MULTIPLE_UP_KEYS: [KeyCode; 1] = [KeyCode::PageUp];

// Other
/// Keys that, when pressed, act as an enter key.
/// Composed of `RIGHT_KEYS` and `KeyCode::Enter`.
pub const ENTER_KEYS: [KeyCode; 4] = [
    KeyCode::Enter,
    KeyCode::Right,
    KeyCode::Char('d'),
    KeyCode::Char('l'),
]; // Mutate this according to the RIGHT_KEYS const.
/// Keys that, when pressed, act as an escape key.
pub const ESCAPE_KEYS: [KeyCode; 2] = [KeyCode::Esc, KeyCode::Char('q')];
