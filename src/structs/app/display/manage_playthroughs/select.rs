//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{BORDER_NOT_SELECTED, BORDER_SELECTED},
    enums::{
        MainMenuEnum, ManagePlaythroughsMenu, ManagePlaythroughsSelected as Selected,
        PlaythroughsSortBy as SortBy, VerticalAlignment,
    },
    structs::{App, Playthrough, PlaythroughName},
    utils::{create_block, create_popup, create_popup_area},
};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};
use std::rc::Rc;
use tokio::time::Duration;

impl App {
    pub fn display_manage_playthroughs_select(&self, area: Rect, buf: &mut Buffer) {
        let MainMenuEnum::ManagePlaythroughs(ManagePlaythroughsMenu::Select {
            selected,
            sort_ascending,
            sort_by,
        }) = self.menu().current()
        else {
            unreachable!()
        };
        if self.account.playthroughs.is_empty() {
            let (paragraph, popup_area): (Paragraph, Rect) = create_popup(
                area,
                VerticalAlignment::Middle,
                Alignment::Center,
                20,
                50,
                Some("Warning"),
                "You have no game saves.",
            );
            paragraph
                .alignment(Alignment::Center)
                .render(popup_area, buf);
            return;
        };
        // Get selected playthrough name, if one,
        let selected_playthrough_name: Option<PlaythroughName> =
            if let Selected::Playthroughs { selected } = selected {
                Some(selected.clone())
            } else {
                None
            };
        // Create the main block and area that everything else will live inside
        let main_block: Block = create_block(None::<&str>, 1).padding(Padding {
            left: 4,
            right: 4,
            top: 1,
            bottom: 1,
        });
        let area: Rect = main_block.inner(create_popup_area(
            area,
            VerticalAlignment::Middle,
            Alignment::Center,
            100,
            70,
        ));
        let main_area: Rect = main_block.inner(area);
        // Display the main block
        main_block.render(area, buf);
        // Get the parts of the main area
        let (sorting_options_area, playthroughs_block_area): (Rect, Rect) = {
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(main_area);
            (parts[0], parts[1])
        };
        // Render sorting options
        {
            let horizontal_padding: u16 = 1;
            let parts: Rc<[Rect]> = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(
                        (sort_by.as_str_user().chars().count()
                            + usize::from(horizontal_padding) * 2
                            + 2)
                        .try_into()
                        .unwrap(),
                    ),
                    Constraint::Min(0),
                    Constraint::Length(
                        (if *sort_ascending {
                            "Ascending"
                        } else {
                            "Descending"
                        }
                        .chars()
                        .count()
                            + usize::from(horizontal_padding) * 2
                            + 2)
                        .try_into()
                        .unwrap(),
                    ),
                ])
                .split(sorting_options_area);
            // Render the sort_by text
            {
                let sort_by_area: Rect = Rect {
                    x: parts[0].x,
                    y: parts[0].y,
                    width: parts[0].width,
                    height: parts[0].height,
                };
                let sort_by_block: Block = Block::bordered()
                    .padding(Padding::horizontal(horizontal_padding))
                    .border_type(if selected.is_sort_by() {
                        BORDER_SELECTED
                    } else {
                        BORDER_NOT_SELECTED
                    });
                let sort_by_paragraph: Paragraph =
                    Paragraph::new(sort_by.as_str_user()).block(sort_by_block);
                sort_by_paragraph.render(sort_by_area, buf);
            }
            // Render the sort_ascending text
            {
                let sort_ascending_area: Rect = Rect {
                    x: parts[2].x,
                    y: parts[2].y,
                    width: parts[2].width,
                    height: parts[2].height,
                };
                let sort_ascending_block: Block = Block::bordered()
                    .padding(Padding::horizontal(horizontal_padding))
                    .border_type(if selected.is_sort_ascending() {
                        BORDER_SELECTED
                    } else {
                        BORDER_NOT_SELECTED
                    });
                let sort_ascending_paragraph: Paragraph = Paragraph::new(if *sort_ascending {
                    "Ascending"
                } else {
                    "Descending"
                })
                .block(sort_ascending_block);
                sort_ascending_paragraph.render(sort_ascending_area, buf);
            }
        }
        // Create the playthroughs block and area that the playthroughs will get displayed in
        let playthroughs_block: Block =
            create_block(Some("Playthroughs"), 1).border_type(if selected.is_playthroughs() {
                BORDER_SELECTED
            } else {
                BORDER_NOT_SELECTED
            });
        let playthroughs_area: Rect = playthroughs_block.inner(playthroughs_block_area);
        playthroughs_block.render(playthroughs_block_area, buf);
        // Render the playthroughs one by one
        // Get the sorted playthroughs
        let playthroughs_sorted: Vec<(&PlaythroughName, &Playthrough)> =
            self.account.playthroughs.sorted(*sort_by, *sort_ascending);
        // Calculate how much scroll offset there is
        let scroll_offset: usize = playthroughs_sorted
            .iter()
            .position(
                |(playthrough_name, _): &(&PlaythroughName, &Playthrough)| -> bool {
                    if let Selected::Playthroughs { selected } = selected {
                        selected == *playthrough_name
                    } else {
                        false
                    }
                },
            )
            .unwrap_or(0);
        // How much space there's left
        let mut vertical_real_estate: u16 = playthroughs_area.height;
        let horizontal_real_estate: u16 = playthroughs_area.width;
        let mut y_offset: u16 = playthroughs_area.y;
        let mut displayed: usize = 0;
        // Display the playthroughs one by one
        for (playthrough_name, playthrough) in playthroughs_sorted.iter().skip(scroll_offset) {
            // Get the text and block of the current playthrough (not the area).
            let (text, block): (Text, Block) = playthrough.as_displayable(
                if let Some(name) = &selected_playthrough_name {
                    *name == **playthrough_name
                } else {
                    false
                },
                playthrough_name,
            );
            // If the text is too big to be displayed, it means there's no more vertical space on
            // the screen. That's calculated as:
            //     text.height() (self-explanatory)
            //     + 4 (the block's barrier and padding)
            //     + 1 (the line after the block should be empty)
            //     + 1 (the guarantee that the message "Displaying elemets .. to .." can display)
            if usize::from(vertical_real_estate) < text.height() + 4 + 1 + 1 {
                break;
            };
            // Define the area
            let current_playthrough_area: Rect = {
                let width: u16 = (text.width() + 4).try_into().unwrap();
                let x: u16 = playthroughs_area.x + (horizontal_real_estate - width) / 2;
                Rect {
                    x,
                    y: y_offset,
                    width,
                    height: (text.height() + 4).try_into().unwrap(),
                }
            };
            y_offset += current_playthrough_area.height + 1;
            vertical_real_estate -= current_playthrough_area.height + 1;
            displayed += 1;
            Paragraph::new(text)
                .block(block)
                .render(current_playthrough_area, buf);
        }
        Paragraph::new(format!(
            "Displaying {displayed} elements ({}-{}) out of {}",
            scroll_offset + 1,
            scroll_offset + displayed,
            playthroughs_sorted.len()
        ))
        .italic()
        .dim()
        .render(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(playthroughs_area)[1],
            buf,
        );
    }
}
