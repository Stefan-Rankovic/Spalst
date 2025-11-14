use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Clear, Widget},
};

use crate::{
    enums::{MainMenu, Menu},
    structs::App,
};

impl App {
    pub fn main_menu_display(&self, area: Rect, buf: &mut Buffer) {
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.");
        };
        match main_menu {
            MainMenu::Base(_) => self.main_menu_base_display(area, buf),
            MainMenu::CreatePlaythrough(_, _) => {
                self.main_menu_create_playthrough_display(area, buf)
            }
            MainMenu::LoadPlaythrough => self.main_menu_load_playthrough_display(area, buf),
            MainMenu::Achievements => self.main_menu_achievements_display(area, buf),
            MainMenu::Quit => Clear.render(area, buf),
            _ => todo!(),
        };
    }
}
