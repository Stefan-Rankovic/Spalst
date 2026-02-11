//! SPDX-License-Identifier: GPL-3.0-only
use ratatui::{
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, block::Title},
};

/// Creates a bordered (with rounded borders) block with uniform padding as passed and a title, if
/// one is passed.
pub fn create_block<'a, T>(title: Option<T>, padding: u16) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    // Create a new Block
    let mut block: Block = Block::bordered()
        .border_set(border::ROUNDED)
        .padding(Padding::uniform(padding));
    // If there should be a title, enter the if
    if let Some(title) = title {
        // Get the actual Title, because otherwise nothing can be done because the type of title is
        // unknown.
        let mut t: Title = title.into();
        // Get the content of the title.
        let content: String = t.content.to_string();
        // If the content doesn't have whitespaces around, add them
        if content.trim() == content {
            t.content = format!(" {} ", content).into();
        };
        // Add the title to the block.
        block = block.title(t);
        // Center the title
        block = block.title_alignment(Alignment::Center);
    };
    // Return the block
    block
}
