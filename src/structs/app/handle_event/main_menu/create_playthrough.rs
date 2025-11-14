use crate::{
    enums::{MainMenu, Menu},
    structs::{App, Playthrough},
};
use color_eyre::eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode};

impl App {
    pub fn main_menu_create_playthrough_handle_event(&mut self, event: Event) -> Result<()> {
        // Ensure that self.menu is Menu::Main
        let Menu::Main(ref mut main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main");
        };
        // Ensure that self.menu is Menu::Main(MainMenu::CreatePlaythrough)
        let MainMenu::CreatePlaythrough(current_input, warning_displayed) = main_menu else {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::CreatePlaythrough)");
        };
        if let Event::Key(key) = event {
            if *warning_displayed {
                *warning_displayed = false;
            } else {
                match key.code {
                    KeyCode::Backspace => {
                        if !current_input.is_empty() {
                            *current_input = current_input[..current_input.len() - 1].to_string();
                        };
                    }
                    KeyCode::Char(c) => current_input.push(c),
                    KeyCode::Esc => *main_menu = MainMenu::default(),
                    KeyCode::Enter => {
                        // Just keep this line here unless you want to fight the borrow checker.
                        let input: String = current_input.clone();
                        // Check if there's already a playthrough with the same name. If there is,
                        // display a warning
                        if self.account().playthroughs.contains_key(&input) {
                            // No, doing "*warning_displayed = true" won't work.
                            self.menu = Menu::Main(MainMenu::CreatePlaythrough(input, true));
                        } else {
                            // No, doing "*main_menu = MainMenu::default()" won't work.
                            self.menu = Menu::Main(MainMenu::default());
                            self.account_mut()
                                .playthroughs
                                .insert(input, Playthrough::default());
                        };
                    }
                    _ => {}
                };
            };
        };
        // Ok.
        Ok(())
    }
}
