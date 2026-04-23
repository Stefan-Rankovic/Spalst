//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::BlockDisplay, traits::Renderable};
use ratatui::{buffer::Buffer, layout::Rect};

/// Previously you could render things. Now you can put them in a box (`Block`) and THEN render them!
pub trait Blockable: Renderable {
    fn is_in_block(&self) -> bool {
        self.get_block().is_some()
    }
    #[must_use]
    fn get_block(&self) -> Option<&BlockDisplay>;
    /// Returns the previous `Block` (if one) and sets the new one.
    fn set_block(
        &mut self,
        new_block: BlockDisplay,
    ) -> Option<BlockDisplay>;

    fn render_with_block(
        &self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        if let Some(block) = self.get_block() {
            let inner_area: Rect = block.inner(area);
            block.basic_render(area, buf);
            self.basic_render(inner_area, buf);
        } else {
            self.basic_render(area, buf);
        }
    }
}
