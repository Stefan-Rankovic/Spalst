use crate::{
    enums::{MainMenu, Menu},
    structs::App,
};
use color_eyre::eyre::Result;
use ratatui::crossterm::event::{Event, KeyCode};

impl App {
    pub fn main_menu_base_handle_event(&mut self, event: Event) -> Result<()> {
        let Menu::Main(ref mut main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.");
        };
        let MainMenu::Base(selected) = main_menu else {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::Base).");
        };
        let page_keys_change_by: u8 = 3;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => main_menu.select_prev()?,
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                    main_menu.select_next()?
                }
                KeyCode::PageUp => {
                    for _ in 0..page_keys_change_by {
                        main_menu.select_prev()?;
                    }
                }
                KeyCode::PageDown => {
                    for _ in 0..page_keys_change_by {
                        main_menu.select_next()?;
                    }
                }
                KeyCode::Home => *main_menu = MainMenu::base_from_first(),
                KeyCode::End => *main_menu = MainMenu::base_from_last(),
                KeyCode::Char('q') => {
                    if *main_menu == MainMenu::base_from(MainMenu::Quit) {
                        *main_menu = MainMenu::Quit;
                    } else {
                        *main_menu = MainMenu::base_from(MainMenu::Quit)
                    }
                }
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                    *main_menu = *selected.clone()
                }
                _ => {}
            };
        };
        // Ok.
        Ok(())
    }
}
