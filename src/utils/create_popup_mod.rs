//! SPDX-License-Identifier: GPL-3.0-only
use crate::{enums::VerticalAlignment, utils::create_block};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Paragraph, block::Title},
};
use std::rc::Rc;

pub fn create_popup<'a, Tl, Tx>(
    area: Rect,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: Alignment,
    vertical_percentage: u16,
    horizontal_percentage: u16,
    title: Option<Tl>,
    message: Tx,
) -> (Paragraph<'a>, Rect)
where
    Tl: Into<Title<'a>>,
    Tx: Into<Text<'a>>,
{
    // Define the vertical and horizontal constraints
    let ((constraints_vertical, index_vertical), (constraints_horizontal, index_horizontal)): (
        (Vec<Constraint>, usize),
        (Vec<Constraint>, usize),
    ) = (
        match vertical_alignment {
            VerticalAlignment::Top => (
                vec![
                    Constraint::Percentage(vertical_percentage),
                    Constraint::Percentage(100 - vertical_percentage),
                ],
                0,
            ),
            VerticalAlignment::Middle => (
                vec![
                    Constraint::Percentage((100 - vertical_percentage) / 2),
                    Constraint::Percentage(vertical_percentage),
                    Constraint::Percentage((100 - vertical_percentage) / 2),
                ],
                1,
            ),
            VerticalAlignment::Bottom => (
                vec![
                    Constraint::Percentage(100 - vertical_percentage),
                    Constraint::Percentage(vertical_percentage),
                ],
                1,
            ),
        },
        match horizontal_alignment {
            Alignment::Left => (
                vec![
                    Constraint::Percentage(horizontal_percentage),
                    Constraint::Percentage(100 - horizontal_percentage),
                ],
                0,
            ),
            Alignment::Center => (
                vec![
                    Constraint::Percentage((100 - horizontal_percentage) / 2),
                    Constraint::Percentage(horizontal_percentage),
                    Constraint::Percentage((100 - horizontal_percentage) / 2),
                ],
                1,
            ),
            Alignment::Right => (
                vec![
                    Constraint::Percentage(100 - horizontal_percentage),
                    Constraint::Percentage(horizontal_percentage),
                ],
                1,
            ),
        },
    );
    // According to the vertical constraints, define the parts of the screen.
    let vertical_parts: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_vertical)
        .split(area);
    // According to the horizontal constraints, define the parts of the screen for the
    // index_vertical's index of vertical_parts.
    let horizontal_parts: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints_horizontal)
        .split(*vertical_parts.get(index_vertical).unwrap());
    // The popup area
    let popup_area: Rect = *horizontal_parts.get(index_horizontal).unwrap();
    // Create a Paragraph that has the text of the message argument passed, inside a block with the
    // title of the title argument passed.
    let paragraph: Paragraph = Paragraph::new(message).block(create_block(title, 1));
    // Ok.
    (paragraph, popup_area)
}
