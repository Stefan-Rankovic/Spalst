//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{AppEvent, MenuThingPinnedStatus, action::Action},
    structs::BlockDisplay,
    traits::{Blockable, MenuThing, Renderable, Styled},
};
use async_trait::async_trait;
use core::fmt::Debug;
use derive_new::new;
use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use spalst_derives::{Blockable, Styled};
use spalst_updater::enums::ProgramUpdatePhase;

#[derive(Blockable, Debug, Styled, new)]
pub struct UpdateMT {
    #[new(default)]
    style: Style,

    #[new(default)]
    block: Option<BlockDisplay>,

    is_selectable: bool,
    is_selected: bool,
    pinned_status: MenuThingPinnedStatus,

    phase: ProgramUpdatePhase,
}

impl Renderable for UpdateMT {
    fn basic_render(
        &self,
        _area: Rect,
        _buf: &mut Buffer,
    ) {
        match self.phase {
            _ => todo!(),
        }
    }
}

#[async_trait]
impl MenuThing for UpdateMT {
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
        _app_event: AppEvent,
    ) -> Vec<Action> {
        todo!()
    }
}
