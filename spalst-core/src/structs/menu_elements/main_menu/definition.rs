//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::MenuElements;
use ratatui::style::Style;
use spalstatui::{structs::Block, traits::Styled};

#[derive(Debug)]
pub struct MainMenu {
    pub(in crate::structs::menu_elements::main_menu) selectable: bool,
    pub(in crate::structs::menu_elements::main_menu) selected:
        Option<<Self as MenuElements>::Elements>,

    pub(in crate::structs::menu_elements::main_menu) block: Option<Block>,

    style: Style,

    pub last_played_available: bool,
    playthroughs_exist: bool,
}

impl MainMenu {
    pub const fn new(
        selectable: bool,
        selected: Option<<Self as MenuElements>::Elements>,
        last_played_available: bool,
        playthroughs_exist: bool,
    ) -> Self {
        Self {
            selectable,
            selected,
            block: None,
            style: Style::new(),
            last_played_available,
            playthroughs_exist,
        }
    }
    pub const fn styled(
        selectable: bool,
        selected: Option<<Self as MenuElements>::Elements>,
        style: Style,
        last_played_available: bool,
        playthroughs_exist: bool,
    ) -> Self {
        Self {
            selectable,
            selected,
            block: None,
            style,
            last_played_available,
            playthroughs_exist,
        }
    }
}

impl Styled for MainMenu {
    fn style(&self) -> Style {
        self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
