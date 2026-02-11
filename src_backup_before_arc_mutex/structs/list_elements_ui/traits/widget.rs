//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::ListElementsUi,
    traits::{AsDisplayable, SortMethod as SortMethodTrait, Sorter as SorterTrait},
    utils::create_block,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Widget},
};
use std::rc::Rc;

impl<
    'h,
    't,
    ElementType: Copy + Eq,
    DataElementType: AsDisplayable,
    SortMethod: SortMethodTrait,
    Sorter: SorterTrait<SortMethod>,
> Widget for ListElementsUi<'h, 't, ElementType, DataElementType, SortMethod, Sorter>
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_area: Rect = if self.display_block {
            let main_block: Block = create_block(self.title, 1);
            let main_area: Rect = main_block.inner(area);
            main_block.render(area, buf);
            main_area
        } else {
            area
        };

        let elements_area: Rect = if self.display_sort_options {
            let (sorting_options_area, elements_area): (Rect, Rect) = {
                let parts: Rc<[Rect]> = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(2 + 1), Constraint::Min(0)])
                    .split(main_area);
                (parts[0], parts[1])
            };
            self.render_sort_options(sorting_options_area, buf);
            elements_area
        } else {
            main_area
        };

        self.render_elements(elements_area, buf);
    }
}
