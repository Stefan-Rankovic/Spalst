use crate::{
    enums::{MainMenu, Menu},
    structs::App,
};
use color_eyre::eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode};

impl App {
    pub fn main_menu_achievements_handle_event(&self, event: Event) -> Result<()> {
        // Ensure that self.menu is Menu::Main
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main");
        };
        // Ensure that self.menu is Menu::Main(MainMenu::Achievements)
        if *main_menu != MainMenu::Achievements {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::Achievements)");
        };
        if let Event::Key(key) = event {
            match key.code {
                _ => todo!(),
            };
        };
        // Ok.
        Ok(())
    }
}
