//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{ListElementsUiEnum, MainMenuEnum, ManagePlaythroughsMenu, VerticalAlignment},
    structs::{App, PlaythroughId},
    utils::{create_popup, create_popup_area},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Paragraph, Widget},
};

impl<'a> App {
    pub fn display_manage_playthroughs_select(&'a self, area: Rect, buf: &mut Buffer) {
        let MainMenuEnum::ManagePlaythroughs(ManagePlaythroughsMenu::Select(ui)) =
            self.menu().current()
        else {
            unreachable!()
        };

        // If there are no playthroughs.
        if self.account.playthroughs.is_empty() {
            let (paragraph, popup_area): (Paragraph, Rect) = create_popup(
                area,
                VerticalAlignment::Middle,
                Alignment::Center,
                20,
                50,
                Some("Warning"),
                "You have no game saves.",
            );
            paragraph
                .alignment(Alignment::Center)
                .render(popup_area, buf);
            return;
        };

        // Get selected playthrough ID, if one.
        let selected_playthrough_id: Option<PlaythroughId> =
            if let Some(ListElementsUiEnum::Elements { selected }) = ui.selected {
                Some(selected)
            } else {
                None
            };

        let main_area: Rect =
            create_popup_area(area, VerticalAlignment::Middle, Alignment::Center, 100, 70);

        ui.render(main_area, buf);
    }
}
