//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::style::{BORDER_NOT_SELECTED, BORDER_SELECTED},
    structs::ListElementsUi,
    traits::{AsDisplayable, SortMethod as SortMethodTrait, Sorter as SorterTrait},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    widgets::{Block, Padding, Paragraph, Widget},
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
    pub fn render_sort_options(&self, area: Rect, buf: &mut Buffer) {
        let horizontal_padding: u16 = 1;
        let parts: Rc<[Rect]> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(
                    (self.sorter.sort_method().as_str_user().chars().count()
                        + usize::from(horizontal_padding) * 2
                        + 2)
                    .try_into()
                    .unwrap(),
                ),
                Constraint::Min(0),
                Constraint::Length(
                    (if *self.sorter.sort_ascending() {
                        "Ascending"
                    } else {
                        "Descending"
                    }
                    .chars()
                    .count()
                        + usize::from(horizontal_padding) * 2
                        + 2)
                    .try_into()
                    .unwrap(),
                ),
            ])
            .split(area);
        // Render the sort_method text
        {
            let sort_method_area: Rect = Rect {
                x: parts[0].x,
                y: parts[0].y,
                width: parts[0].width,
                height: parts[0].height,
            };
            let sort_method_block: Block = Block::bordered()
                .padding(Padding::horizontal(horizontal_padding))
                .border_type(
                    if let Some(selected) = self.selected
                        && selected.is_sort_method()
                    {
                        BORDER_SELECTED
                    } else {
                        BORDER_NOT_SELECTED
                    },
                );
            Paragraph::new(self.sorter.sort_method().as_str_user())
                .block(sort_method_block)
                .render(sort_method_area, buf);
        }
        // Render the sort_ascending text
        {
            let sort_ascending_area: Rect = Rect {
                x: parts[2].x,
                y: parts[2].y,
                width: parts[2].width,
                height: parts[2].height,
            };
            let sort_ascending_block: Block = Block::bordered()
                .padding(Padding::horizontal(horizontal_padding))
                .border_type(
                    if let Some(selected) = self.selected
                        && selected.is_sort_ascending()
                    {
                        BORDER_SELECTED
                    } else {
                        BORDER_NOT_SELECTED
                    },
                );
            let sort_ascending_paragraph: Paragraph =
                Paragraph::new(if *self.sorter.sort_ascending() {
                    "Ascending"
                } else {
                    "Descending"
                })
                .block(sort_ascending_block);
            sort_ascending_paragraph.render(sort_ascending_area, buf);
        }
    }
}
