use crate::{enums::MainMenuEnum, structs::App, utils::create_block};
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};

impl App {
    pub fn display(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        // Ok.
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block: Block = create_block(Some("Spalst"), 1);
        // The current area is not good to work with because it allows the passed function to
        // modify the Block's lines, title, etc. which isn't indended behaviour. That's fixed by
        // creating a new Rect object that only contains the inner parts of the Block and pass that
        // object as the area argument to the rendering functions.
        let inner_area = block.inner(area);
        // To avoid issues where the Block is rendered in its own inner area (because the
        // rendering functions can't access the actual area where the block should be rendered),
        // render the block now.
        block.render(area, buf);
        match self.menu().current() {
            MainMenuEnum::Browsing => unreachable!(),
            MainMenuEnum::CreatePlaythrough { .. } => {
                self.display_create_playthrough(inner_area, buf)
            }
            MainMenuEnum::LoadPlaythrough => self.display_create_playthrough(inner_area, buf),
            MainMenuEnum::Achievements => self.display_create_playthrough(inner_area, buf),
            MainMenuEnum::Settings => self.display_create_playthrough(inner_area, buf),
            MainMenuEnum::Quit => {}
        }
    }
}
