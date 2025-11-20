//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::ACHIEVEMENT_DISPLAY_TIME,
    enums::{MainMenuEnum, VerticalAlignment},
    structs::{Achievement, App},
    utils::{create_block, create_popup},
};
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Widget},
};

impl App {
    pub fn display(&self, terminal: &mut DefaultTerminal) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        // Ok.
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block: Block = create_block(Some("Spalst"), 1);
        // The current area is not good to work with because it allows the passed function to
        // modify the Block's lines, title, etc., which isn't indended behaviour. That's fixed by
        // creating a new Rect object that only contains the inner parts of the Block and pass that
        // object as the area argument to the rendering functions.
        let inner_area = block.inner(area);
        // To avoid issues where the Block is rendered in its own inner area (because the
        // rendering functions can't access the actual area where the block should be rendered),
        // render the block now.
        block.render(area, buf);
        // Display achievements gotten
        if let Some(achievement) = self.display_achievements_queue.current() {
            let (paragraph, popup_area) = create_popup(
                inner_area,
                VerticalAlignment::Top,
                Alignment::Right,
                12,
                20,
                Some("Notification"),
                vec![
                    Line::from(format!(
                        "Acquired achievement \"{}\"!",
                        achievement.name_user()
                    )),
                    Line::from(""),
                    Line::from(format!(
                        "{:.1}",
                        self.display_achievements_queue.seconds_left().unwrap(),
                    )),
                ],
            );
            paragraph
                .alignment(Alignment::Center)
                .render(popup_area, buf);
        }
        match self.menu().current() {
            MainMenuEnum::Browsing => self.display_browsing(inner_area, buf),
            MainMenuEnum::CreatePlaythrough { .. } => {
                self.display_create_playthrough(inner_area, buf)
            }
            MainMenuEnum::LoadPlaythrough => self.display_load_playthrough(inner_area, buf),
            MainMenuEnum::Achievements => self.display_achievements(inner_area, buf),
            MainMenuEnum::Settings => self.display_settings(inner_area, buf),
            MainMenuEnum::Quit => {}
        }
    }
}
