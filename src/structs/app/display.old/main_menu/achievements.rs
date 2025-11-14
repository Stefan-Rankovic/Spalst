use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::{
    enums::{MainMenu, Menu},
    structs::App,
    utils::{create_block, create_popup},
};

impl App {
    pub fn main_menu_achievements_display(&self, area: Rect, buf: &mut Buffer) {
        // Ensure that self.menu is Menu::Main
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main");
        };
        // Ensure that self.menu is Menu::Main(MainMenu::Achievements)
        if *main_menu != MainMenu::Achievements {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::Achievements)");
        };
        let (_, achievements_area): (Paragraph, Rect) =
            create_popup(area, 90, 50, Some("Achievements"), "");
        todo!();
    }
}
