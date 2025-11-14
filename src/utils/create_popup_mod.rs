use crate::utils::create_block;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Paragraph, block::Title},
};
use std::rc::Rc;

pub fn create_popup<'a, Tl, Tx>(
    area: Rect,
    vertical_percentage: u16,
    horizontal_percentage: u16,
    title: Option<Tl>,
    message: Tx,
) -> (Paragraph<'a>, Rect)
where
    Tl: Into<Title<'a>>,
    Tx: Into<Text<'a>>,
{
    // Create 3 vertical parts. The second one takes up vertical_percentage percent of the screen.
    // The other two take up the surrounding areas.
    let vertical_parts: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - vertical_percentage) / 2),
            Constraint::Percentage(vertical_percentage),
            Constraint::Percentage((100 - vertical_percentage) / 2),
        ])
        .split(area);
    // Create 3 horizontal parts inside the middle vertical part. The second one takes up
    // horizontal_percentage of the screen. The other two take up the surrounding areas.
    let horizontal_parts: Rc<[Rect]> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - horizontal_percentage) / 2),
            Constraint::Percentage(horizontal_percentage),
            Constraint::Percentage((100 - horizontal_percentage) / 2),
        ])
        .split(vertical_parts[1]);
    // The second element in horizontal_parts is the popup_area needed.
    let popup_area: Rect = horizontal_parts[1];
    // Create a Paragraph that has the text of the message argument passed, inside a block with the
    // title of the title argument passed.
    let paragraph: Paragraph = Paragraph::new(message).block(create_block(title, 1));
    // Ok.
    (paragraph, popup_area)
}
