use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, crossterm::event::Event};

use crate::{
    enums::{MainMenu, Menu},
    structs::App,
};

impl App {
    pub fn main_menu_handle_event(
        &mut self,
        event: Event,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.");
        };
        if let Event::Resize(_, _) = event {
            self.display(terminal)?;
        };
        match main_menu {
            MainMenu::Base(_) => self.main_menu_base_handle_event(event),
            MainMenu::CreatePlaythrough(_, _) => {
                self.main_menu_create_playthrough_handle_event(event)
            }
            MainMenu::LoadPlaythrough => self.main_menu_load_playthrough_handle_event(event),
            MainMenu::Achievements => self.main_menu_achievements_handle_event(event),
            _ => todo!(),
        }
    }
}
