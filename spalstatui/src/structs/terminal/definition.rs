//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Frame;
use color_eyre::eyre::Result;
use ratatui::{
    backend::ClearType,
    buffer::Buffer,
    layout::{Position, Rect, Size},
    prelude::{Backend as _, CrosstermBackend},
};
use std::io::{Stdout, stdout};

/// The terminal.
#[derive(Debug)]
pub struct Terminal {
    /// The backend.
    pub(crate) backend: CrosstermBackend<Stdout>,
    /// The two `Buffer`s.
    pub(crate) buffers: [Buffer; 2],
    /// The current `Buffer`.
    ///
    /// Note: this could absolutely be a `bool` or even `u8`. It is `usize` instead because it's
    /// like that in ratatui, and somebody probably already thought out why it has to be `usize`.
    pub(crate) current: usize,
    /// Whether the cursor is hidden.
    pub(crate) hidden_cursor: bool,
    area: Rect,
    /// The last known area (`Rect`).
    pub(crate) last_known_area: Rect,
    /// The last known cursor `Position`.
    last_known_cursor_pos: Position,
    /// The `Frame` count until now.
    pub(crate) frame_count: usize,
}

impl Terminal {
    /// Returns a new `Terminal` instance.
    ///
    /// # Errors
    /// If `CrosstermBackend::new(std::io::stdout()).size()` fails.
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let area: Rect = backend.size()?.into();
        Ok(Self {
            backend,
            buffers: [Buffer::empty(area), Buffer::empty(area)],
            current: 0,
            hidden_cursor: false,
            area,
            last_known_area: area,
            last_known_cursor_pos: Position::ORIGIN,
            frame_count: 0,
        })
    }

    /// Gets the current `Frame`.
    pub const fn get_frame(&mut self) -> Frame<'_> {
        Frame {
            cursor_pos: None, // I have no idea why this is `None`.
            area: self.area,
            count: self.frame_count, // count has to go before buf due to a lifetime error.
            buf: self.current_buffer_mut(),
        }
    }

    /// Gets the current `Buffer` as a mutable reference.
    #[must_use]
    pub const fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }
    #[must_use]
    /// Gets the current `Buffer`.
    pub const fn backend(&self) -> &CrosstermBackend<Stdout> {
        &self.backend
    }
    /// Gets the current `Backend` as a mutable reference.
    pub const fn backend_mut(&mut self) -> &mut CrosstermBackend<Stdout> {
        &mut self.backend
    }

    /// Flushes the terminal.
    ///
    /// # Errors
    /// If drawing to the terminal fails.
    pub fn flush(&mut self) -> Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some(&(col, row, _)) = updates.last() {
            self.last_known_cursor_pos = Position { x: col, y: row };
        }
        self.backend.draw(updates.into_iter())?;
        // Ok.
        Ok(())
    }

    /// Resizes the terminal.
    ///
    /// # Errors
    /// If `self.clear()` fails.
    pub fn resize(&mut self, area: Rect) -> Result<()> {
        self.set_area(area);
        self.clear()?;
        self.last_known_area = area;
        // Ok.
        Ok(())
    }

    fn set_area(&mut self, area: Rect) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.area = area;
    }

    /// Resizes the terminal if the current `self.size()` is not the last known area.
    ///
    /// # Errors
    /// If `self.size()` fails or `self.resize()` fails.
    pub fn autoresize(&mut self) -> Result<()> {
        let area: Rect = self.size()?.into();
        if area != self.last_known_area {
            self.resize(area)?;
        }
        // Ok.
        Ok(())
    }

    /// Hides the cursor.
    ///
    /// # Errors
    /// If the cursor could not be hidden.
    pub fn hide_cursor(&mut self) -> Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        // Ok.
        Ok(())
    }
    /// Shows the cursor.
    ///
    /// # Errors
    /// If the cursor could not be shown.
    pub fn show_cursor(&mut self) -> Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        // Ok.
        Ok(())
    }
    /// Gets the cursor position.
    ///
    /// # Errors
    /// If the cursor position could not be gotten.
    pub fn get_cursor_position(&mut self) -> Result<Position> {
        Ok(self.backend.get_cursor_position()?)
    }
    /// Sets the cursor position.
    ///
    /// # Errors
    /// If the cursor position could not be set.
    pub fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<()> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        // Ok.
        Ok(())
    }

    /// Clears the terminal.
    ///
    /// # Errors
    /// If the terminal screen could not be cleared.
    pub fn clear(&mut self) -> Result<()> {
        self.backend.clear_region(ClearType::All)?;
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        // Ok.
        Ok(())
    }

    /// Swaps the buffers.
    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    /// Gets the size of the terminal.
    ///
    /// # Errors
    /// If the terminal size could not be gotten.
    pub fn size(&self) -> Result<Size> {
        // Ok.
        Ok(self.backend.size()?)
    }
}
