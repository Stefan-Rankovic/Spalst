//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Frame, Terminal};
use color_eyre::eyre::{Ok, Report, Result};
use ratatui::{layout::Position, prelude::Backend as _};

impl Terminal {
    /// Draws to the terminal.
    ///
    /// # Errors
    /// Fails if `self.try_draw() fails`.
    pub async fn draw<F>(&mut self, rendering_function: F) -> Result<()>
    where
        F: AsyncFnOnce(&mut Frame<'_>),
    {
        self.try_draw(async |frame: &mut Frame<'_>| {
            rendering_function(frame).await;
            Ok(())
        })
        .await
    }

    /// Tries to draw to the terminal.
    ///
    /// # Errors
    /// If `self.autoresize()` fails.
    /// If the passed `rendering_function` fails.
    /// If `self.flush()` fails.
    /// If the cursor position, after the call to `rendering_function`, is:
    ///     - `None`: if `self.hide_cursor()` fails.
    ///     - `Some`: if `self.show_cursor()` or `self.set_cursor_position()` fail.
    /// If `self.flush()` fails again, after the cursor is done.
    pub async fn try_draw<F, E>(&mut self, rendering_function: F) -> Result<()>
    where
        F: AsyncFnOnce(&mut Frame<'_>) -> Result<(), E>,
        E: Into<Report>,
    {
        // Autoresize; otherwise OOB.
        self.autoresize()?;

        let mut frame: Frame<'_> = self.get_frame();

        rendering_function(&mut frame).await.map_err(Into::into)?;

        // The frame holds a mutable reference to self, but self.flush() needs that too, and access
        // to frame will be required after, so the cursor position is saved here and the frame is
        // dropped.
        let cursor_pos: Option<Position> = frame.cursor_pos;
        // The frame is now dropped, as it's not used again.

        // Draw to stdout
        self.flush()?;

        match cursor_pos {
            None => self.hide_cursor()?,
            Some(position) => {
                self.show_cursor()?;
                self.set_cursor_position(position)?;
            }
        }

        self.swap_buffers();

        // Flush
        self.backend.flush()?;

        // increment frame count before returning from draw
        self.frame_count = self.frame_count.wrapping_add(1);

        // Ok.
        Ok(())
    }
}
