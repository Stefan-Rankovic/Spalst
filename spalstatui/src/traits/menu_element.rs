//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuEvent,
    structs::Block,
    traits::{Styled, WidgetRef},
};
use core::{any::Any, fmt::Debug, pin::Pin};
use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{HorizontalAlignment, Rect},
};

/// A menu element. Used for displaying things on the screen.
pub trait MenuElement: Any + Debug + Send + Styled + Sync {
    fn selectable(&self) -> bool;
    fn selected(&self) -> bool;

    fn block(&self) -> Option<&Block>;
    fn set_block(&mut self, block: Block) -> Option<Block>;
    #[must_use = "This method consumes self and returns it."]
    fn with_block(mut self, block: Block) -> (Self, Option<Block>)
    where
        Self: Sized,
    {
        let old_block: Option<Block> = self.set_block(block);
        (self, old_block)
    }

    fn as_any(&self) -> &dyn Any;

    /// Whether anything will be done if the passed event is handled or not.
    ///
    /// # Examples
    /// ```rust
    /// #use spalstatui::{
    ///     enums::MenuEvent,
    ///     structs::EmptyME,
    ///     traits::MenuElement,
    /// }
    /// struct ItemList {
    ///     items: Vec<u8>,
    ///     selected_item: usize,
    ///     // ...
    ///     #empty: EmptyME,
    /// }
    ///
    /// impl MenuElement for ItemList {
    ///     #fn selectable(&self) -> bool { self.empty.selectable() }
    ///     #fn selected(&self) -> bool { self.empty.selected() }
    ///     #fn block(&self) -> Option<&Block> { self.empty.block() }
    ///     #fn set_block(&mut self, block: Block) -> Option<Block> { self.empty.set_block(block) }
    ///     #fn with_block(mut self, block: Block) -> (Self, Option<Block>) { self.empty.with_block(block) }
    ///     #fn execute_event(&mut self, event: &MenuEvent) { self.empty.execute_event() }
    ///     #fn as_any(&self) -> &dyn Any { self.empty.as_any() }
    ///     #fn render_without_block<'future?(&'future self, area: Rect, buf: &'future mut Buffer) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> { self.empty.render_without_block() }
    ///     // ...
    ///     fn accepts_event(&self, event: &MenuEvent) -> bool {
    ///         // Assuming selecting down on the last index doesn't loop back and the reverse
    ///         match MenuEvent {
    ///             MenuEvent::Nothing => true,
    ///             MenuEvent::SelectLeft | MenuEvent::SelectRight => false,
    ///             MenuEvent::SelectUp => self.selected_item != 0,
    ///             MenuEvent::SelectDown => self.selected_item != self.items.len()
    ///         }
    ///     }
    /// }
    /// ```
    fn accepts_event(&self, event: &MenuEvent) -> bool;
    fn execute_event(&mut self, event: &MenuEvent);

    fn handle_key_event(
        &self,
        key_event: KeyEvent,
    ) -> Pin<Box<dyn Future<Output = MenuEvent> + Send>>;

    fn render_without_block<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>>;
    fn render_with_block<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            if let Some(block) = self.block() {
                let inner_area: Rect = block.inner(area);
                block.render_ref(area, buf).await;
                self.render_without_block(inner_area, buf).await;
            } else {
                self.render_with_block(area, buf).await;
            }
        })
    }
}
