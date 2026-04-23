//! SPDX-License-Identifier: MIT

use crate::structs::Terminal;
use color_eyre::eyre::Result;
use ratatui::{
    backend::Backend as _,
    layout::{Position, Rect, Size},
};

pub fn compute_inline_size(
    backend: &mut Terminal::Backend,
    height: u16,
    size: Size,
    offset_in_previous_viewport: u16,
) -> Result<(Rect, Position)> {
    let pos = backend.get_cursor_position()?;
    let mut row = pos.y;

    let max_height = size.height.min(height);

    let lines_after_cursor = height
        .saturating_sub(offset_in_previous_viewport)
        .saturating_sub(1);

    backend.append_lines(lines_after_cursor)?;

    let available_lines = size.height.saturating_sub(row).saturating_sub(1);
    let missing_lines = lines_after_cursor.saturating_sub(available_lines);
    if missing_lines > 0 {
        row = row.saturating_sub(missing_lines);
    }
    row = row.saturating_sub(offset_in_previous_viewport);

    Ok((
        Rect {
            x: 0,
            y: row,
            width: size.width,
            height: max_height,
        },
        pos,
    ))
}
