use crate::{
    enums::{MainMenu, Menu},
    structs::App,
    utils::create_block,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::rc::Rc;
use strum::IntoEnumIterator;

impl App {
    pub fn main_menu_base_display(&self, area: Rect, buf: &mut Buffer) {
        // Ensure that self.menu is Menu::Main
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.");
        };
        // Ensure that self.menu is Menu::Main(MainMenu::Base)
        let MainMenu::Base(selected) = main_menu else {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::Base).");
        };
        // Check if the Base menu contains another Base.
        if let MainMenu::Base(_) = **selected {
            panic!(
                "MainMenu instance passed had a type of MainMenu::Base(MainMenu::Base(_)) which is not allowed! (this is an error that's not caused by you, the user)"
            );
        };
        // Create centered layout
        let vertical_chunks: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ])
            .split(area);
        let horizontal_chunks: Rc<[Rect]> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(vertical_chunks[1]);
        let menu_area: Rect = horizontal_chunks[1];
        // Calculate spacing for menu items
        let item_height: u16 = 3;
        let menu_items_area: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                MainMenu::iter()
                    .map(|_| Constraint::Length(item_height))
                    .collect::<Vec<_>>(),
            )
            .split(menu_area);
        // Render each menu option
        for (i, option) in MainMenu::iter().skip(1).enumerate() {
            // Get the selected status for the current menu option
            let is_selected: bool = option == **selected;
            // Create the option Block
            let block: Block = create_block(None, 0).border_type(if is_selected {
                BorderType::Double
            } else {
                BorderType::Plain
            });
            // Create a Paragraph inside the created block and render it
            Paragraph::new(option.to_string())
                .block(block)
                .alignment(Alignment::Center)
                .style(if is_selected {
                    Style::default().bold()
                } else {
                    Style::default()
                })
                .render(menu_items_area[i], buf);
        }

        // Render the useful tooltip in the bottom left corner that specifies the controls
        // Define the text that'll get displayed
        let text: Vec<Line> = vec![
            Line::from("Home - Select the first option"),
            Line::from("End - Select the last option"),
            Line::from("↑, w, k - Select upwards"),
            Line::from("Page Up - Multiple select upwards"),
            Line::from("↓, s, j - Select downwards"),
            Line::from("Page Down - Multiple select downwards"),
            Line::from("→, d, l, ⏎ - Choose current selected option"),
            Line::from(format!(
                "q - Select {} (or quit the program if {} is already selected)",
                MainMenu::Quit,
                MainMenu::Quit
            )),
        ];
        // Define the height of the text
        let text_height: u16 = text.len().try_into().unwrap();
        // Define the area where the text will be rendered
        let controls_area: Rect = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(text_height),
            width: area.width,
            height: text_height,
        };
        // Render the text
        Paragraph::new(text)
            .italic()
            .dim()
            .alignment(Alignment::Left)
            .render(controls_area, buf);
    }
}
