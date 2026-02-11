//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{style::Stylize, text::Span};

pub fn none() -> Span<'static> {
    "None".italic().dim()
}
