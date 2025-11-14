use crate::{
    enums::MainMenuEnum,
    structs::App,
    utils::{create_block, create_popup},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, Paragraph, Widget},
};

impl App {
    pub fn display_load_playthrough(&self, area: Rect, buf: &mut Buffer) {
        if *self.menu().current() != MainMenuEnum::LoadPlaythrough {
            unreachable!()
        };
        if self.account.playthroughs.is_empty() {
            let (paragraph, popup_area): (Paragraph, Rect) =
                create_popup(area, 20, 50, Some("Warning"), "You have no game saves.");
            paragraph
                .alignment(Alignment::Center)
                .render(popup_area, buf);
        };
        let playthroughs: Vec<Block> = Vec::new();
        for (playthrough_name, playthrough) in &self.account.playthroughs {
            let current_block: Block = create_block(None::<&str>, 1);
            todo!()
        }
    }
}
