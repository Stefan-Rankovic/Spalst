//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::ListElementsUiEnum,
    structs::ListElementsUi,
    traits::{AsDisplayable, SortMethod as SortMethodTrait, Sorter as SorterTrait},
    types::ItemBlockInfo,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Paragraph, Widget},
};
use std::rc::Rc;

impl<
    'h,
    't,
    ElementType: Copy + Eq,
    DataElementType: AsDisplayable,
    SortMethod: SortMethodTrait,
    Sorter: SorterTrait<SortMethod>,
> ListElementsUi<'h, 't, ElementType, DataElementType, SortMethod, Sorter>
{
    pub fn render_elements(&self, area: Rect, buf: &mut Buffer) {
        // If there are no elements, return early.
        if self.elements.is_empty() {
            return;
        };

        //todo: figure out if this can be ElementType instead of &ElementType.
        let elements: &[(&ElementType, &DataElementType)] = {
            //todo: figure out if this can be ElementType instead of &ElementType.
            let elements_not_sorted: Vec<(&ElementType, &DataElementType)> = self
            .elements
            .iter()
            .map(
                |(key, value): (&ElementType, &DataElementType)| -> (&ElementType, &DataElementType) {
                    (key, value)
                },
            )
            .collect();
            self.sorter.sort_items(&elements_not_sorted)
        };

        // The selected element index.
        let selected_element_index: usize = if let Some(selected_menu_part) = self.selected
            && let ListElementsUiEnum::Elements { selected } = selected_menu_part
        {
            elements
                .iter()
                .position(
                    |(element_id, _): &(&ElementType, &DataElementType)| -> bool {
                        **element_id == selected
                    },
                )
                .unwrap()
        } else {
            0
        };

        // Split the area into the area where the elements should live and the area where the note
        // "Displaying 5 elements (1-5) out of 10" should be, if the arguments say so.
        let (elements_area, selected_note_area): (Rect, Option<Rect>) = if self.display_note {
            let chunks: Rc<[Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area);
            (chunks[0], Some(chunks[1]))
        } else {
            (area, None)
        };

        // Define variables that update as the loop goes on.
        let mut vertical_real_estate: u16 = elements_area.height;
        let horizontal_real_estate: u16 = elements_area.width;
        let mut y_offset: u16 = elements_area.y;
        let mut displayed_count: usize = 0;

        // Loop over elements and display them on the way.
        for (current_element_index, (_, element)) in
            elements.iter().enumerate().skip(selected_element_index)
        {
            // Is the current element selected?
            let is_selected: bool = selected_element_index == current_element_index;

            // Get the text and block (if one).
            let (text, block_info): (Text, Option<ItemBlockInfo>) =
                element.as_displayable(is_selected);

            // The current element's height required for displaying.
            let current_element_height: u16 = (text.height()
                + if let Some((_, (block_height, _))) = block_info {
                    block_height
                } else {
                    0
                })
            .try_into()
            .unwrap();

            // If there's not enough vertical space to display element, exit the loop.
            if current_element_height > vertical_real_estate {
                break;
            };

            // The current element's width
            let current_element_width: u16 = (text.width()
                + if let Some((_, (_, block_width))) = block_info {
                    block_width
                } else {
                    0
                })
            .try_into()
            .unwrap_or(horizontal_real_estate);

            // Center the element horizontally with an x offset.
            let x_offset: u16 = elements_area.x
                + (horizontal_real_estate.saturating_sub(current_element_width)) / 2;

            // Current element's area.
            let current_element_area: Rect = Rect {
                x: x_offset,
                y: y_offset,
                width: current_element_width,
                height: current_element_height,
            };

            // Render current element.
            {
                let mut paragraph: Paragraph = Paragraph::new(text);
                if let Some((block, (_, _))) = block_info {
                    paragraph = paragraph.block(block);
                };
                paragraph.render(current_element_area, buf);
            }

            // Update tracking variables
            let consumed_height: u16 = current_element_area.height + self.element_spacing;
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
                selected_element_index + 1,
                selected_element_index + displayed_count,
                elements.len()
            ))
            .italic()
            .dim()
            .render(selected_note_area, buf);
        };
        todo!();
    }
}
