//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{AppEvent, MenuThingPinnedStatus, action::Action},
    traits::{Blockable, Renderable},
};
use async_trait::async_trait;
use core::{any::Any, fmt::Debug};

/// A thing you have on your screen. Couldn't be simpler.
#[async_trait]
pub trait MenuThing: Any + Blockable + Debug + Renderable + Send + Sync {
    fn is_selectable(&self) -> bool;
    fn is_selected(&self) -> bool;
    fn pinned_status(&self) -> MenuThingPinnedStatus;

    /// Handles the passed `Event`.
    ///
    /// This function will only change the display state; it will not change any internal data.
    /// Internal data is changed according to the return value.
    async fn handle_event(
        &mut self,
        app_event: AppEvent,
    ) -> Vec<Action>;
}
