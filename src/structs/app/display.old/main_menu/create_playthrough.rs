use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    text::{Line, Text},
    widgets::{Paragraph, Widget},
};

use crate::{
    enums::{MainMenu, Menu},
    structs::App,
    utils::create_popup,
};

impl App {
    pub fn main_menu_create_playthrough_display(&self, area: Rect, buf: &mut Buffer) {
        // Ensure that self.menu is Menu::Main
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.");
        };
        // Ensure that self.menu is Menu::Main(MainMenu::CreatePlaythrough)
        let MainMenu::CreatePlaythrough(current_input, warning_displayed) = main_menu else {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::CreatePlaythrough).");
        };
        let (paragraph, popup_area): (Paragraph, Rect);
        if *warning_displayed {
            (paragraph, popup_area) = create_popup(
                area,
                50,
                75,
                Some("Create Playthrough"),
                Line::from(format!(
                    "Playthrough with name \"{current_input}\" already exists!"
                ))
                .red(),
            );
        } else {
            (paragraph, popup_area) = create_popup(
                area,
                50,
                75,
                Some("Create Playthrough"),
                Line::from(format!("Enter name: {current_input}")),
            );
        };
        paragraph
            .alignment(Alignment::Center)
            .render(popup_area, buf);
    }
}
