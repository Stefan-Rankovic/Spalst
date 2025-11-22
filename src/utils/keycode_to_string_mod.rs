//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::crossterm::event::KeyCode;

pub fn keycode_to_string(kc: &KeyCode) -> String {
    match kc {
        KeyCode::Char(c) => c.to_string(),
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
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::BackTab => "Shift+Tab".to_string(),
        _ => unimplemented!(),
    }
}
