//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::style::{BORDER_NOT_SELECTED, BORDER_SELECTED}, enums::{ElementsListSelected, MainMenuEnum, ManagePlaythroughSelected, ManagePlaythroughsMenu, VerticalAlignment}, structs::{App, Playthrough, PlaythroughName, PlaythroughWithName}, traits::AsDisplayable, utils::{create_block, create_popup_area, render_items_in_area}
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

impl App {
    pub fn display_manage_playthroughs_playthrough(&self, area: Rect, buf: &mut Buffer) {
        let MainMenuEnum::ManagePlaythroughs(ManagePlaythroughsMenu::Playthrough { id, selected }) =
            self.menu().current()
        else {
            unreachable!()
        };
        let name: &str = self.account.playthroughs.get_name(*id).unwrap();
        let block_area: Rect =
            create_popup_area(area, VerticalAlignment::Middle, Alignment::Center, 100, 80);
        let block: Block = create_block(Some(name.to_string()), 1);
        let playthrough_area: Rect = block.inner(block_area);
        block.render(block_area, buf);
        let playthrough: &Playthrough = self.account.playthroughs.get(id).unwrap();
        let (info_area, saves_area): (Rect, Rect) = {
            let areas = Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(playthrough_area);
            (areas[0], areas[1])
        };
        let info_block: Block = create_block(None::<&str>, 1);
        let saves_block: Block = create_block(Some("Saves"), 1).border_type(if selected.is_saves() {BORDER_SELECTED} else {BORDER_NOT_SELECTED});
        let mut info_text: Text = Text::from(format!("Name: {}", playthrough.name));
        info_text.extend(playthrough.as_displayable(false).0);
        Paragraph::new(info_text)
            .block(info_block)
            .render(info_area, buf);
        render_items_in_area(saves_area, buf, saves_block, playthrough.saves, , item_spacing, render_note)
        //todo: continue from here yesterday. also, in case of amnesia, which is highly
        //probable, the last change was refactoring the Playthrough struct to use Playtime
        //instead of Duration for storing playtime, also consts were refactored, and look into
        //utils/style as well so maybe move consts/style.rs there idk.
        //also the UI is a big mess. figure something out. nothing is nowhere, at least currently.
        //or, in other words, everything is everywhere.
        todo!();
    }
}
