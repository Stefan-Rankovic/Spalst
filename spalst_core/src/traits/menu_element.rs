//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::MenuEvent;
use core::{any::Any, fmt::Debug, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect},
};
use spalstatui::{
    structs::Block,
    traits::{Styled, WidgetRef},
};

/// A menu element. Used for displaying things on the screen.
pub trait MenuElement: Any + Debug + Send + Styled + Sync + WidgetRef {
    fn selectable(&self) -> bool;
    fn selected(&self) -> bool;

    /// Returns the attached `Block` for this `MenuElement`.
    fn block(&self) -> Option<&Block>;
    /// Sets a new `Block` as the attached `Block`, and returns the old one, if any.
    fn set_block(&mut self, block: Block) -> Option<Block>;
    /// Sets a new `Block` as the attached `Block`, and returns `Self` and the old `Block`, if any.
    #[must_use = "This method consumes self and returns it."]
    fn with_block(mut self, block: Block) -> (Self, Option<Block>)
    where
        Self: Sized,
    {
        let old_block: Option<Block> = self.set_block(block);
        (self, old_block)
    }

    fn as_any(&self) -> &dyn Any;

    fn handle_key_event(
        &self,
        key_event: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = MenuEvent> + Send>>;

    fn render_with_block<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            // todo: see if this works
            if let Some(block) = self.block() {
                let inner_area: Rect = block.inner(area);
                block.render_ref(area, buf).await;
                self.render_ref(inner_area, buf).await;
            } else {
                self.render_ref(area, buf).await;
            }
        })
    }
}
