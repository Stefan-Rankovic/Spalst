//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::MenuElementList, traits::AsDisplayable, types::ItemBlockInfo};
use core::{fmt::Debug, pin::Pin};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize as _,
    text::Text,
    widgets::{Paragraph, Widget as _},
};
use spalstatui::traits::WidgetRef;
use std::rc::Rc;

impl<
    ItemId: Copy + Debug + PartialEq + Send + Sync,
    Item: AsDisplayable + Debug + PartialEq + Send + Sync,
> WidgetRef for MenuElementList<ItemId, Item>
{
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            // If there are no items, return early.
            if self.items.lock().await.is_empty() {
                return;
            }

            // The scroll offset.
            let scroll_offset: usize = if let Some(selected_item_id) = self.selected {
                self.items
                    .lock()
                    .await
                    .iter()
                    .position(|&(item_id, _): &(ItemId, Item)| -> bool {
                        item_id == selected_item_id
                    })
                    .unwrap()
            } else {
                0
            };

            // Split the area into the area where the items should live and the area where the note
            // "Displaying 5 items (1-5) out of 10" should be, if the struct settings say so.
            let (items_area, selected_note_area_option): (Rect, Option<Rect>) =
                if self.show_elements_counter {
                    let chunks: Rc<[Rect]> = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(0), Constraint::Length(1)])
                        .split(area);
                    (chunks[0], Some(chunks[1]))
                } else {
                    (area, None)
                };

            // Define variables that update as the loop goes on.
            let mut vertical_real_estate: u16 = items_area.height;
            let horizontal_real_estate: u16 = items_area.width;
            let mut y_offset: u16 = items_area.y;
            let mut displayed_count: usize = 0;

            // Loop over items and display them on the way.
            for &(current_item_id, ref current_item) in
                self.items.lock().await.iter().skip(scroll_offset)
            {
                // Is the current item selected?
                let is_selected: bool =
                    self.selected
                        .is_some_and(|selected_item_id: ItemId| -> bool {
                            selected_item_id == current_item_id
                        });

                // Get the text and block (if one).
                let (text, block_info): (Text<'_>, Option<ItemBlockInfo<'_>>) =
                    current_item.as_displayable(is_selected);

                // The current item's height required for displaying.
                let current_item_height: u16 = (text.height()
                    + if let Some((_, (block_height, _))) = block_info {
                        block_height
                    } else {
                        0
                    })
                .try_into()
                .unwrap();

                // If there's not enough vertical space to display item, exit the loop.
                if current_item_height > vertical_real_estate {
                    break;
                }

                // The current item's width
                let current_item_width: u16 = (text.width()
                    + if let Some((_, (_, block_width))) = block_info {
                        block_width
                    } else {
                        0
                    })
                .try_into()
                .unwrap_or(horizontal_real_estate);

                // Center the item horizontally with an x offset.
                let x_offset: u16 =
                    items_area.x + (horizontal_real_estate.saturating_sub(current_item_width)) / 2;

                // Current item's area.
                let current_item_area: Rect = Rect {
                    x: x_offset,
                    y: y_offset,
                    width: current_item_width,
                    height: current_item_height,
                };

                // Render current item.
                {
                    let mut paragraph: Paragraph<'_> = Paragraph::new(text);
                    if let Some((block, (_, _))) = block_info {
                        paragraph = paragraph.block(block);
                    }
                    paragraph.render(current_item_area, buf);
                }

                // Update tracking variables.
                let consumed_height: u16 = current_item_area.height + self.item_spacing;
                y_offset += consumed_height;
                vertical_real_estate = vertical_real_estate.saturating_sub(consumed_height);
                displayed_count += 1;
            }

            // Render the note saying how many items and what items were displayed, if there is an area
            // for it to be rendered (if not, it means the struct config says there shouldn't be).
            if let Some(selected_note_area) = selected_note_area_option {
                Paragraph::new(format!(
                    "Displaying {} items ({}-{}) out of {}",
                    displayed_count,
                    scroll_offset + 1,
                    scroll_offset + displayed_count,
                    self.items.lock().await.len()
                ))
                .italic()
                .dim()
                .render(selected_note_area, buf);
            }
            todo!();
        })
    }
}
