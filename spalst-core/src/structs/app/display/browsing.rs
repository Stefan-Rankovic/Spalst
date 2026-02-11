//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{
        BORDER_NOT_SELECTED, BORDER_SELECTED, DOWN_KEYS, ENTER_KEYS, ESCAPE_KEYS,
        MULTIPLE_DOWN_KEYS, MULTIPLE_UP_KEYS, UP_KEYS,
    },
    enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    structs::App,
    utils::{create_block, keycode_to_string, keycodes_to_string},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::rc::Rc;
use strum::IntoDiscriminant;

impl App {
    pub fn display_browsing(&self, area: Rect, buf: &mut Buffer) {
        if self.menu().current().discriminant() != MainMenuEnumDiscriminants::Browsing {
            unreachable!();
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
                MainMenuEnumDiscriminants::iter_no_browsing()
                    .map(|_| Constraint::Length(item_height))
                    .collect::<Vec<_>>(),
            )
            .split(menu_area);
        // Render each menu option
        for (i, option) in MainMenuEnumDiscriminants::iter_no_browsing().enumerate() {
            // Get the selected status for the current menu option
            let selected: bool = self
                .menu()
                .selected()
                .unwrap_or_else(|| panic!(
                    "app.menu changed from the MainMenuEnumDiscriminants::Browsing it was 10 lines of code above, to {}.",
                    self.menu().current().discriminant().as_str_debug(),
                ))
                == option;
            // Create the option Block
            let block: Block = create_block(None::<&str>, 0).border_type(if selected {
                BORDER_SELECTED
            } else {
                BORDER_NOT_SELECTED
            });
            // Create a Paragraph inside the created block and render it
            Paragraph::new(option.as_str_user())
                .block(block)
                .alignment(Alignment::Center)
                .style(if selected {
                    Style::default().bold()
                } else {
                    Style::default()
                })
                .render(menu_items_area[i], buf);
        }

        // Render the useful tooltip in the bottom left corner that specifies the controls
        // Define the text that'll get displayed
        // todo: update this so it's automatic
        let text: Vec<Line> = vec![
            Line::from("Home - Select the first option"),
            Line::from(format!(
                "End, {} - Select the last option",
                keycodes_to_string(&ESCAPE_KEYS)
            )),
            Line::from(format!("{} - Select upwards", keycodes_to_string(&UP_KEYS))),
            Line::from(format!(
                "{} - Multiple select upwards",
                keycodes_to_string(&MULTIPLE_UP_KEYS),
            )),
            Line::from(format!(
                "{} - Select downwards",
                keycodes_to_string(&DOWN_KEYS)
            )),
            Line::from(format!(
                "{} - Multiple select downwards",
                keycodes_to_string(&MULTIPLE_DOWN_KEYS),
            )),
            Line::from(format!(
                "{} - Choose current selected option",
                keycodes_to_string(&ENTER_KEYS)
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
