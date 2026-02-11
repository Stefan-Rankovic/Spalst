//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::crossterm::event::KeyCode;

pub fn keycode_to_string(kc: KeyCode) -> String {
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Everything else is unimplemented by design."
    )]
    match kc {
        KeyCode::Char(char) => char.to_string(),
        KeyCode::Enter => "⏎".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PgUp".to_string(),
        KeyCode::PageDown => "PgDn".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::F(function_number) => format!("F{function_number}"),
        KeyCode::BackTab => "Shift+Tab".to_string(),
        _ => unimplemented!(),
    }
}
