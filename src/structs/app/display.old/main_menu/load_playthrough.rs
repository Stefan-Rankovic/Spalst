use crate::{
    enums::{MainMenu, Menu},
    structs::App,
    utils::{create_block, create_popup_mod::create_popup},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Widget,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

impl App {
    //todo: add comments to this whole function explaining what the code does
    pub fn main_menu_load_playthrough_display(&self, area: Rect, buf: &mut Buffer) {
        let Menu::Main(ref main_menu) = self.menu else {
            unreachable!("Expected self.menu to be Menu::Main.")
        };
        if *main_menu != MainMenu::LoadPlaythrough {
            unreachable!("Expected self.menu to be Menu::Main(MainMenu::LoadingPlaythrough).")
        };
        if self.account().playthroughs.is_empty() {
            let (paragraph, popup_area): (Paragraph, Rect) = create_popup(
                area,
                20,
                50,
                Some("Notice"),
                "You have no game saves in the current save file.",
            );
            paragraph
                .alignment(Alignment::Center)
                .render(popup_area, buf);
            return;
        };
        // We got to here which means there's at least one playthrough.
        todo!();
    }
}

// fn todo_check_if_this_works_to_display_chosen_saves(
// self_: App,
// area: Rect,
// buf: &mut Buffer,
// block: Block,
// n: usize, // Current save number that the user entered (still didn't press enter to lock it in
// yet)
// ) {
// Get the inner area of the block (removes borders and padding)
// let inner: Rect = block.inner(area);
// Get the text regarding saves to be loaded
// let loading_text: Text = Text::from(&self_.account.as_ref().unwrap().games);
// Load it
// Paragraph::new(loading_text).block(block).render(area, buf);
// Get the last (bottom) row of the inner area
// let bottom_right_area: Rect = Rect {
// x: inner.x, // Start at the left edge of the inner area - that's fine
// Subtract one from the
// height, also adding inner.y
// to it (because the box may
// not start at y=0)
// y: inner.y + inner.height.saturating_sub(1),
// width: inner.width, // The full width is fine
// height: 1,          // Only 1 row tall - the last row
// };
// let n_text: Line = Line::from(n.to_string().italic());
// Render the text
// Paragraph::new(n_text)
// .alignment(Alignment::Right) // Make it appear in the bottom right corner
// .render(bottom_right_area, buf);
// }
