use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Widget,
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::structs::App;

impl App {
    pub fn loading_account_display(&self, area: Rect, buf: &mut Buffer) {
        // Define the text to be displayed
        let loading_text: Text = Text::from("Loading account...");
        // Display the text
        Paragraph::new(loading_text).render(area, buf)
    }
}
