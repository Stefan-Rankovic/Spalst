//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::CREATE_PLAYTHROUGH_WARN_TIME,
    enums::{MainMenuEnum, VerticalAlignment},
    structs::App,
    utils::create_popup,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Widget},
};

impl App {
    pub fn display_create_playthrough(&self, area: Rect, buf: &mut Buffer) {
        let MainMenuEnum::CreatePlaythrough {
            current_input,
            warning_displayed_on,
        } = self.menu().current()
        else {
            unreachable!();
        };
        let (paragraph, popup_area): (Paragraph, Rect);
        if let Some(instant) = warning_displayed_on {
            (paragraph, popup_area) = create_popup(
                area,
                VerticalAlignment::Middle,
                Alignment::Center,
                20,
                50,
                Some("Warning"),
                vec![
                    Line::from(format!(
                        "Playthrough with the name \"{current_input}\" already exists!"
                    ))
                    .red(),
                    Line::from(""),
                    Line::from(format!(
                        "{:.1}",
                        CREATE_PLAYTHROUGH_WARN_TIME - instant.elapsed().as_secs_f64()
                    )),
                ],
            );
        } else {
            (paragraph, popup_area) = create_popup(
                area,
                VerticalAlignment::Middle,
                Alignment::Center,
                15,
                40,
                Some("Create Playthrough"),
                format!("Enter name: {current_input}"),
            );
        };
        paragraph
            .alignment(Alignment::Center)
            .render(popup_area, buf);
    }
}
