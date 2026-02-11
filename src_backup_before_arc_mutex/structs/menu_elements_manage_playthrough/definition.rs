//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::traits::{MenuElement, MenuElements};

/// todo
#[derive(Debug)]
pub struct MenuElementsManagePlaythrough {
    selectable: bool,
    selected: bool,
    pub selected_element: <Self as MenuElements>::Elements,
}

impl MenuElement for MenuElementsManagePlaythrough {
    fn selectable(&self) -> bool {
        self.selectable
    }
    fn selected(&self) -> bool {
        self.selected
    }
}

impl Widget for MenuElementsManagePlaythrough {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_elements(area, buf);
    }
}
