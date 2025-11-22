//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::VerticalAlignment,
    utils::{create_block, create_popup_area},
};
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
    // Ok.
    (
        Paragraph::new(message).block(create_block(title, 1)),
        create_popup_area(
            area,
            vertical_alignment,
            horizontal_alignment,
            vertical_percentage,
            horizontal_percentage,
        ),
    )
}
