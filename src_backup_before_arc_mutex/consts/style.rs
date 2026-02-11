//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{
    style::{Style, Stylize},
    text::Span,
    widgets::BorderType,
};

// Borders
pub const BORDER_NOT_SELECTED: BorderType = BorderType::Plain;
pub const BORDER_SELECTED: BorderType = BorderType::Double;
