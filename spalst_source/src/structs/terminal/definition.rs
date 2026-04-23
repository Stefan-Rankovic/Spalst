//! SPDX-License-Identifier: MIT

use crate::utils::compute_inline_size;
use color_eyre::eyre::{Ok, Report, Result};
use ratatui::{
    Viewport,
    backend::ClearType,
    buffer::{Buffer, Cell},
    layout::{Position, Rect, Size},
    prelude::{Backend as _, CrosstermBackend},
};
use std::io::Stdout;

/// For any documentation, check [docs.rs](https://docs.rs/ratatui-core/0.1.0/ratatui_core/terminal/struct.Terminal.html).
#[derive(Debug)]
pub struct Terminal {
    backend: CrosstermBackend<Stdout>,
    buffers: [Buffer; 2],
    current: usize,
    pub(in crate::structs::terminal) hidden_cursor: bool,
    viewport: Viewport,
    viewport_area: Rect,
    last_known_area: Rect,
    last_known_cursor_pos: Position,
    frame_count: usize,
}

impl Terminal {
    pub type Backend = CrosstermBackend<Stdout>;

    pub const VIEWPORT: Viewport = Viewport::Fullscreen;

    pub fn new(mut backend: Self::Backend) -> Result<Self> {
        let area = match Self::VIEWPORT {
            Viewport::Fullscreen | Viewport::Inline(_) => backend.size()?.into(),
            Viewport::Fixed(area) => area,
        };
        let (viewport_area, cursor_pos) = match Self::VIEWPORT {
            Viewport::Fullscreen => (area, Position::ORIGIN),
            Viewport::Inline(height) => compute_inline_size(&mut backend, height, area.as_size(), 0)?,
            Viewport::Fixed(viewport_area) => (viewport_area, area.as_position()),
        };
        // Ok.
        Ok(Self {
            backend,
            buffers: [Buffer::empty(viewport_area), Buffer::empty(viewport_area)],
            current: 0,
            hidden_cursor: false,
            viewport: Self::VIEWPORT,
            viewport_area,
            last_known_area: area,
            last_known_cursor_pos: cursor_pos,
            frame_count: 0,
        })
    }

    pub const fn get_frame(&mut self) -> Frame<'_> {
        let count = self.frame_count;
        Frame::new(None, self.viewport_area, self.current_buffer_mut(), count)
    }

    pub const fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current]
    }

    pub const fn backend(&self) -> &Self::Backend {
        &self.backend
    }

    pub const fn backend_mut(&mut self) -> &mut Self::Backend {
        &mut self.backend
    }

    pub fn flush(&mut self) -> Result<()> {
        let previous_buffer = &self.buffers[1 - self.current];
        let current_buffer = &self.buffers[self.current];
        let updates = previous_buffer.diff(current_buffer);
        if let Some(&(col, row, _)) = updates.last() {
            self.last_known_cursor_pos = Position { x: col, y: row };
        }
        self.backend.draw(updates.into_iter()).map_err(Report::from)
    }

    pub fn resize(
        &mut self,
        area: Rect,
    ) -> Result<()> {
        let next_area = match self.viewport {
            Viewport::Inline(height) => {
                let offset_in_previous_viewport = self
                    .last_known_cursor_pos
                    .y
                    .saturating_sub(self.viewport_area.top());
                compute_inline_size(
                    &mut self.backend,
                    height,
                    area.as_size(),
                    offset_in_previous_viewport,
                )?
                .0
            }
            Viewport::Fixed(_) | Viewport::Fullscreen => area,
        };
        self.set_viewport_area(next_area);
        self.clear()?;

        self.last_known_area = area;
        // Ok.
        Ok(())
    }

    fn set_viewport_area(
        &mut self,
        area: Rect,
    ) {
        self.buffers[self.current].resize(area);
        self.buffers[1 - self.current].resize(area);
        self.viewport_area = area;
    }

    pub fn autoresize(&mut self) -> Result<()> {
        // fixed viewports do not get autoresized
        if matches!(self.viewport, Viewport::Fullscreen | Viewport::Inline(_)) {
            let area = self.size()?.into();
            if area != self.last_known_area {
                self.resize(area)?;
            }
        }
        // Ok.
        Ok(())
    }

    pub async fn draw<F>(
        &mut self,
        render_callback: F,
    ) -> Result<CompletedFrame<'_>>
    where
        F: AsyncFnOnce(&mut Frame<'_>),
    {
        self.try_draw(async |frame| {
            render_callback(frame).await;
            // Ok.
            Ok(())
        })
        .await
    }

    pub async fn try_draw<F, E>(
        &mut self,
        render_callback: F,
    ) -> Result<CompletedFrame<'_>>
    where
        F: AsyncFnOnce(&mut Frame<'_>) -> Result<(), E>,
        E: Into<Report>,
    {
        // Autoresize - otherwise we get glitches if shrinking or potential desync between widgets
        // and the terminal (if growing), which may OOB.
        self.autoresize()?;

        let mut frame = self.get_frame();

        render_callback(&mut frame).await.map_err(Into::into)?;

        // We can't change the cursor position right away because we have to flush the frame to
        // stdout first. But we also can't keep the frame around, since it holds a &mut to
        // Buffer. Thus, we're taking the important data out of the Frame and dropping it.
        let cursor_position = frame.get_cursor_position();

        // Draw to stdout
        self.flush()?;

        match cursor_position {
            None => self.hide_cursor()?,
            Some(position) => {
                self.show_cursor()?;
                self.set_cursor_position(position)?;
            }
        }

        self.swap_buffers();

        // Flush
        self.backend.flush()?;

        let completed_frame = CompletedFrame {
            buffer: &self.buffers[1 - self.current],
            area: self.last_known_area,
            count: self.frame_count,
        };

        // increment frame count before returning from draw
        self.frame_count = self.frame_count.wrapping_add(1);

        // Ok.
        Ok(completed_frame)
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        // Ok.
        Ok(())
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        // Ok.
        Ok(())
    }

    #[deprecated = "use `get_cursor_position()` instead which returns `Result<Position>`"]
    pub fn get_cursor(&mut self) -> Result<(u16, u16)> {
        let Position { x, y } = self.get_cursor_position()?;
        // Ok.
        Ok((x, y))
    }

    #[deprecated = "use `set_cursor_position((x, y))` instead which takes `impl Into<Position>`"]
    pub fn set_cursor(
        &mut self,
        x: u16,
        y: u16,
    ) -> Result<()> {
        self.set_cursor_position(Position { x, y })
    }

    pub fn get_cursor_position(&mut self) -> Result<Position> {
        self.backend.get_cursor_position().map_err(Report::from)
    }

    pub fn set_cursor_position<P: Into<Position>>(
        &mut self,
        position: P,
    ) -> Result<()> {
        let position = position.into();
        self.backend.set_cursor_position(position)?;
        self.last_known_cursor_pos = position;
        // Ok.
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        match self.viewport {
            Viewport::Fullscreen => self.backend.clear_region(ClearType::All)?,
            Viewport::Inline(_) => {
                self.backend
                    .set_cursor_position(self.viewport_area.as_position())?;
                self.backend.clear_region(ClearType::AfterCursor)?;
            }
            Viewport::Fixed(_) => {
                let area = self.viewport_area;
                for y in area.top()..area.bottom() {
                    self.backend.set_cursor_position(Position { x: 0, y })?;
                    self.backend.clear_region(ClearType::AfterCursor)?;
                }
            }
        }
        // Reset the back buffer to make sure the next update will redraw everything.
        self.buffers[1 - self.current].reset();
        // Ok.
        Ok(())
    }

    pub fn swap_buffers(&mut self) {
        self.buffers[1 - self.current].reset();
        self.current = 1 - self.current;
    }

    pub fn size(&self) -> Result<Size> {
        self.backend.size().map_err(Report::from)
    }

    pub fn insert_before<F>(
        &mut self,
        height: u16,
        draw_fn: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut Buffer),
    {
        match self.viewport {
            #[cfg(feature = "scrolling-regions")]
            Viewport::Inline(_) => self.insert_before_scrolling_regions(height, draw_fn),
            #[cfg(not(feature = "scrolling-regions"))]
            Viewport::Inline(_) => self.insert_before_no_scrolling_regions(height, draw_fn),
            _ => Ok(()),
        }
    }

    #[expect(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::as_conversions,
        reason = "I copied this from Ratatui so it's probably fine."
    )]
    #[cfg(not(feature = "scrolling-regions"))]
    fn insert_before_no_scrolling_regions(
        &mut self,
        height: u16,
        draw_fn: impl FnOnce(&mut Buffer),
    ) -> Result<()> {
        // The approach of this function is to first render all of the lines to insert into a
        // temporary buffer, and then to loop drawing chunks from the buffer to the screen. drawing
        // this buffer onto the screen.
        let area = Rect {
            x: 0,
            y: 0,
            width: self.viewport_area.width,
            height,
        };
        let mut buffer = Buffer::empty(area);
        draw_fn(&mut buffer);
        let mut buffer = buffer.content.as_slice();

        // Use i32 variables so we don't have worry about overflowed u16s when adding, or about
        // negative results when subtracting.
        let mut drawn_height: i32 = self.viewport_area.top().into();
        let mut buffer_height: i32 = height.into();
        let viewport_height: i32 = self.viewport_area.height.into();
        let screen_height: i32 = self.last_known_area.height.into();

        // The algorithm here is to loop, drawing large chunks of text (up to a screen-full at a
        // time), until the remainder of the buffer plus the viewport fits on the screen. We choose
        // this loop condition because it guarantees that we can write the remainder of the buffer
        // with just one call to Self::draw_lines().
        while buffer_height + viewport_height > screen_height {
            // We will draw as much of the buffer as possible on this iteration in order to make
            // forward progress. So we have:
            //
            //     to_draw = min(buffer_height, screen_height)
            //
            // We may need to scroll the screen up to make room to draw. We choose the minimal
            // possible scroll amount so we don't end up with the viewport sitting in the middle of
            // the screen when this function is done. The amount to scroll by is:
            //
            //     scroll_up = max(0, drawn_height + to_draw - screen_height)
            //
            // We want `scroll_up` to be enough so that, after drawing, we have used the whole
            // screen (drawn_height - scroll_up + to_draw = screen_height). However, there might
            // already be enough room on the screen to draw without scrolling (drawn_height +
            // to_draw <= screen_height). In this case, we just don't scroll at all.
            let to_draw = buffer_height.min(screen_height);
            let scroll_up = 0_i32.max(drawn_height + to_draw - screen_height);
            self.scroll_up(scroll_up as u16)?;
            buffer = self.draw_lines((drawn_height - scroll_up) as u16, to_draw as u16, buffer)?;
            drawn_height += to_draw - scroll_up;
            buffer_height -= to_draw;
        }

        // There is now enough room on the screen for the remaining buffer plus the viewport,
        // though we may still need to scroll up some of the existing text first. It's possible
        // that by this point we've drained the buffer, but we may still need to scroll up to make
        // room for the viewport.
        //
        // We want to scroll up the exact amount that will leave us completely filling the screen.
        // However, it's possible that the viewport didn't start on the bottom of the screen and
        // the added lines weren't enough to push it all the way to the bottom. We deal with this
        // case by just ensuring that our scroll amount is non-negative.
        //
        // We want:
        //   screen_height = drawn_height - scroll_up + buffer_height + viewport_height
        // Or, equivalently:
        //   scroll_up = drawn_height + buffer_height + viewport_height - screen_height
        let scroll_up = 0.max(drawn_height + buffer_height + viewport_height - screen_height);
        self.scroll_up(scroll_up as u16)?;
        self.draw_lines(
            (drawn_height - scroll_up) as u16,
            buffer_height as u16,
            buffer,
        )?;
        drawn_height += buffer_height - scroll_up;

        self.set_viewport_area(Rect {
            y: drawn_height as u16,
            ..self.viewport_area
        });

        // Clear the viewport off the screen. We didn't clear earlier for two reasons. First, it
        // wasn't necessary because the buffer we drew out of isn't sparse, so it overwrote
        // whatever was on the screen. Second, there is a weird bug with tmux where a full screen
        // clear plus immediate scrolling causes some garbage to go into the scrollback.
        self.clear()?;

        // Ok.
        Ok(())
    }

    #[cfg(feature = "scrolling-regions")]
    fn insert_before_scrolling_regions(
        &mut self,
        mut height: u16,
        draw_fn: impl FnOnce(&mut Buffer),
    ) -> Result<()> {
        // The approach of this function is to first render all of the lines to insert into a
        // temporary buffer, and then to loop drawing chunks from the buffer to the screen. drawing
        // this buffer onto the screen.
        let area = Rect {
            x: 0,
            y: 0,
            width: self.viewport_area.width,
            height,
        };
        let mut buffer = Buffer::empty(area);
        draw_fn(&mut buffer);
        let mut buffer = buffer.content.as_slice();

        // Handle the special case where the viewport takes up the whole screen.
        if self.viewport_area.height == self.last_known_area.height {
            // "Borrow" the top line of the viewport. Draw over it, then immediately scroll it into
            // scrollback. Do this repeatedly until the whole buffer has been put into scrollback.
            let mut first = true;
            while !buffer.is_empty() {
                buffer = if first {
                    self.draw_lines(0, 1, buffer)?
                } else {
                    self.draw_lines_over_cleared(0, 1, buffer)?
                };
                first = false;
                self.backend.scroll_region_up(0..1, 1)?;
            }

            // Redraw the top line of the viewport.
            let width = self.viewport_area.width as usize;
            let top_line = self.buffers[1 - self.current].content[0..width].to_vec();
            self.draw_lines_over_cleared(0, 1, &top_line)?;
            // Ok.
            return Ok(());
        }

        // Handle the case where the viewport isn't yet at the bottom of the screen.
        {
            let viewport_top = self.viewport_area.top();
            let viewport_bottom = self.viewport_area.bottom();
            let screen_bottom = self.last_known_area.bottom();
            if viewport_bottom < screen_bottom {
                let to_draw = height.min(screen_bottom - viewport_bottom);
                self.backend
                    .scroll_region_down(viewport_top..viewport_bottom + to_draw, to_draw)?;
                buffer = self.draw_lines_over_cleared(viewport_top, to_draw, buffer)?;
                self.set_viewport_area(Rect {
                    y: viewport_top + to_draw,
                    ..self.viewport_area
                });
                height -= to_draw;
            }
        }

        let viewport_top = self.viewport_area.top();
        while height > 0 {
            let to_draw = height.min(viewport_top);
            self.backend.scroll_region_up(0..viewport_top, to_draw)?;
            buffer = self.draw_lines_over_cleared(viewport_top - to_draw, to_draw, buffer)?;
            height -= to_draw;
        }

        // Ok.
        Ok(())
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::min_ident_chars,
        clippy::as_conversions,
        clippy::integer_division,
        reason = "I copied this from Ratatui so it's probably fine."
    )]
    fn draw_lines<'cells>(
        &mut self,
        y_offset: u16,
        lines_to_draw: u16,
        cells: &'cells [Cell],
    ) -> Result<&'cells [Cell]> {
        let width: usize = self.last_known_area.width.into();
        let (to_draw, remainder) = cells.split_at(width * lines_to_draw as usize);
        if lines_to_draw > 0 {
            let iter = to_draw
                .iter()
                .enumerate()
                .map(|(i, c)| ((i % width) as u16, y_offset + (i / width) as u16, c));
            self.backend.draw(iter)?;
            self.backend.flush()?;
        }
        // Ok.
        Ok(remainder)
    }

    #[cfg(feature = "scrolling-regions")]
    fn draw_lines_over_cleared<'a>(
        &mut self,
        y_offset: u16,
        lines_to_draw: u16,
        cells: &'a [Cell],
    ) -> Result<&'a [Cell]> {
        let width: usize = self.last_known_area.width.into();
        let (to_draw, remainder) = cells.split_at(width * lines_to_draw as usize);
        if lines_to_draw > 0 {
            let area = Rect::new(0, y_offset, width as u16, y_offset + lines_to_draw);
            let old = Buffer::empty(area);
            let new = Buffer {
                area,
                content: to_draw.to_vec(),
            };
            self.backend.draw(old.diff(&new).into_iter())?;
            self.backend.flush()?;
        }
        // Ok.
        Ok(remainder)
    }

    #[cfg(not(feature = "scrolling-regions"))]
    fn scroll_up(
        &mut self,
        lines_to_scroll: u16,
    ) -> Result<()> {
        if lines_to_scroll > 0 {
            self.set_cursor_position(Position::new(
                0,
                self.last_known_area.height.saturating_sub(1),
            ))?;
            self.backend.append_lines(lines_to_scroll)?;
        }
        // Ok.
        Ok(())
    }
}
