//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{
        AppEvent,
        MenuThingPinnedStatus,
        action::{Action, DisplayManagerAction, UpdateAction},
    },
    structs::{BlockDisplay, menu_thing::UpdateMT},
    traits::{Blockable, MenuThing, Renderable, Styled},
    utils::{block, block_dimensions},
};
use async_trait::async_trait;
use core::{ops::Div as _, time::Duration};
use crossterm::event::{Event, KeyCode};
use derive_new::new;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Styled as _},
    text::Text,
    widgets::{Paragraph, Widget as _},
};
use spalst_derives::{Blockable, Styled};
use spalst_updater::enums::ProgramUpdatePhase;
use tokio::time::Instant;

/// The welcome screen.
#[derive(Blockable, Debug, Styled, new)]
pub struct WelcomeMT {
    #[new(default)]
    style: Style,

    #[new(default)]
    block: Option<BlockDisplay>,

    is_selectable: bool,
    is_selected: bool,
    pinned_status: MenuThingPinnedStatus,

    constructed_at: Instant,
}

impl Renderable for WelcomeMT {
    fn basic_render(
        &self,
        area: Rect,
        buf: &mut Buffer,
    ) {
        const BASE_BLOCK_WIDTH: u16 = block_dimensions().0;
        const BASE_BLOCK_HEIGHT: u16 = block_dimensions().1;

        const WELCOME_TEXT: &str = "Welcome!";
        const WELCOME_TEXT_HEIGHT: u16 = 1;
        const TOTAL_WELCOME_HEIGHT: u16 = WELCOME_TEXT_HEIGHT + BASE_BLOCK_HEIGHT;
        let welcome_text_width: u16 = u16::try_from(WELCOME_TEXT.len()).unwrap();
        let total_welcome_width: u16 = welcome_text_width + BASE_BLOCK_WIDTH;
        let welcome_text_area: Rect = area.centered(
            Constraint::Length(total_welcome_width),
            Constraint::Length(TOTAL_WELCOME_HEIGHT),
        );
        Paragraph::new(WELCOME_TEXT)
            .block(block())
            .centered()
            .render(welcome_text_area, buf);

        // Only display "Press any key to continue..." if enough time passed.
        if self.constructed_at.elapsed() > Self::TIME_BEFORE_CONTINUE {
            const TEXT: &str = "Press any key to continue...";
            const TEXT_HEIGHT: u16 = 1;
            const TOTAL_HEIGHT: u16 = TEXT_HEIGHT;
            const STYLE: Style = Style::new().dim().italic();
            let text_width: u16 = u16::try_from(TEXT.len()).unwrap();
            let total_width: u16 = text_width;
            let text_area: Rect = {
                let space_below_welcome_text: u16 = area.bottom() - welcome_text_area.bottom();
                // The text will be placed 20% (1/5) below the welcome text.
                let y_offset: u16 = space_below_welcome_text.div(5);
                // Not needed, but I like using Rect::centered(), so this becomes needed.
                // And, as a bonus, this makes it more future proof in case the text becomes
                // multiple lines or it gets a Block.
                let around_text_area: Rect = Rect {
                    y: welcome_text_area.bottom(),
                    height: 2 * y_offset + TOTAL_HEIGHT,
                    ..area
                };
                // The center of the previous Rect is the actual area where this should be placed.
                around_text_area.centered(
                    Constraint::Length(total_width),
                    Constraint::Length(TOTAL_HEIGHT),
                )
            };
            Paragraph::new(Text::from(TEXT).set_style(STYLE))
                .centered()
                .render(text_area, buf);
        }
    }
}

#[async_trait]
impl MenuThing for WelcomeMT {
    fn is_selectable(&self) -> bool {
        self.is_selectable
    }

    fn is_selected(&self) -> bool {
        self.is_selected
    }

    fn pinned_status(&self) -> MenuThingPinnedStatus {
        self.pinned_status
    }

    async fn handle_event(
        &mut self,
        app_event: AppEvent,
    ) -> Vec<Action> {
        let AppEvent::Event(Event::Key(key_event)) = app_event else {
            return Vec::new();
        };

        // Condition:
        //
        //      [continue message appeared] OR [Esc] => enter update screen
        //      (and the implied condition that a key was pressed; look at lines above)
        //
        // Which means that if the time for the "Press any key to continue..." message to appear
        // passed, the program will continue. Or if the key is Escape, even if the message didn't
        // appear yet.
        let condition_esc: bool = key_event.code == KeyCode::Esc;
        let condition_continue: bool = self.constructed_at.elapsed() >= Self::TIME_BEFORE_CONTINUE;
        if !(condition_esc || condition_continue) {
            return Vec::new();
        }
        let select_new_mt_action: Action = Action::DisplayManagerAction(DisplayManagerAction::SelectNewMT(Box::new(UpdateMT::new(
            true,
            true,
            MenuThingPinnedStatus::NotPinned,
            ProgramUpdatePhase::CheckingForUpdates,
        ))));

        let start_first_phase_action: Action = Action::UpdateAction(UpdateAction::StartFirstPhase);

        vec![select_new_mt_action, start_first_phase_action]
    }
}

impl WelcomeMT {
    /// The time before displaying "Press any key to continue...".
    pub const TIME_BEFORE_CONTINUE: Duration = Duration::from_secs(5);
}
