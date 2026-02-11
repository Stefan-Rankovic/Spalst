//! SPDX-License-Identifier: GPL-3.0-only

use crate::{traits::AsDisplayable, types::ItemBlockInfo};
use color_eyre::eyre::{Result, bail};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};
use std::rc::Rc;

#[deprecated = "Use ListElementsUi::render() instead (with Widget), or directly with render_elements."]
pub fn render_items_in_area<'a, T>(
    area: Rect,
    buf: &mut Buffer,
    block: Option<Block<'a>>,
    items: &'a [T],
    selected_item_index: Option<usize>,
    item_spacing: u16,
    render_note: bool,
) -> Result<()>
where
    T: AsDisplayable,
{
    // If the selected index is bigger than the length of the items, bail
    if let Some(selected_index) = selected_item_index
        && selected_index > items.len().saturating_sub(0)
    {
        bail!(
            "Passed selected index ({selected_index}) is bigger than the list allows (the length is {}).",
            items.len()
        );
    };

    // If there is a block, render it now and set the new area to the inner of the block.
    let area: Rect = if let Some(ref block) = block {
        let area_new: Rect = block.inner(area);
        block.render(area, buf);
        area_new
    } else {
        area
    };

    // If there are no items, return early.
    if items.is_empty() {
        // Ok.
        return Ok(());
    };

    // The item scroll offset.
    let scroll_offset: usize = selected_item_index.unwrap_or(0);

    // Split the area into the area where the items should live and the area where the note
    // "Displaying 5 elements (1-5) out of 10" should be, if the arguments say so.
    let (items_area, selected_note_area): (Rect, Option<Rect>) = if render_note {
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
    for (current_item_index, item) in items.iter().enumerate().skip(scroll_offset) {
        // Is the current item selected?
        let is_selected: bool = selected_item_index == Some(current_item_index);

        // Get the text and block (if one).
        let (text, block_info): (Text, Option<ItemBlockInfo>) = item.as_displayable(is_selected);

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
        };

        // The current item's width
        let current_item_width: u16 = (text.width()
            + if let Some((_, (_, block_width))) = block_info {
                block_width
            } else {
                0
            })
        .try_into()
        .unwrap_or(horizontal_real_estate);

        // Center the item horizontally with an x offset
        let x_offset: u16 =
            items_area.x + (horizontal_real_estate.saturating_sub(current_item_width)) / 2;

        // Current item's area
        let current_item_area: Rect = Rect {
            x: x_offset,
            y: y_offset,
            width: current_item_width,
            height: current_item_height,
        };

        // Render current item
        {
            let mut paragraph: Paragraph = Paragraph::new(text);
            if let Some((block, (_, _))) = block_info {
                paragraph = paragraph.block(block);
            };
            paragraph.render(current_item_area, buf);
        }

        // Update tracking variables
        let consumed_height: u16 = current_item_area.height + item_spacing;
        y_offset += consumed_height;
        vertical_real_estate = vertical_real_estate.saturating_sub(consumed_height);
        displayed_count += 1;
    }

    // Render the note saying how many elements and what elements were displayed, if the arguments
    // say so.
    if let Some(selected_note_area) = selected_note_area {
        Paragraph::new(format!(
            "Displaying {} elements ({}-{}) out of {}",
            displayed_count,
            scroll_offset + 1,
            scroll_offset + displayed_count,
            items.len()
        ))
        .italic()
        .dim()
        .render(selected_note_area, buf);
    };

    // Ok.
    Ok(())
}
