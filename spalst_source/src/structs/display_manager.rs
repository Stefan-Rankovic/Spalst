//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{
        AppEvent,
        action::{Action, DisplayManagerAction},
    },
    traits::{MenuThing, Renderable, Styled},
    utils::block,
};
use color_eyre::eyre::{OptionExt as _, Result};
use core::any::{TypeId, type_name};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Widget as _},
};

/// Manages the display.
///
/// Apart from just displaying this, it has some useful things like history.
#[derive(Debug)]
pub struct DisplayManager {
    /// History of `MenuThing`s (first element is the oldest).
    menu_things: Vec<Box<dyn MenuThing>>,
    /// Currently displayed `MenuThing` (by index).
    current: usize,
    /// The `Block` that surrounds the entire screen.
    pub screen_block: Option<Block<'static>>,
}

/// Does nothing.
impl Styled for DisplayManager {
    fn get_style(&self) -> Style {
        Style::new()
    }

    fn set_style(
        &mut self,
        _new_style: Style,
    ) -> Style {
        Style::new()
    }
}

impl Renderable for DisplayManager {
    fn basic_render(
        &self,
        mut area: Rect,
        buf: &mut Buffer,
    ) {
        if let Some(screen_block) = self.screen_block.as_ref() {
            let inner_area: Rect = screen_block.inner(area);
            screen_block.render(area, buf);
            area = inner_area;
        }
        self.current_menu_thing().render_with_block(area, buf);
    }
}

impl From<Box<dyn MenuThing>> for DisplayManager {
    fn from(value: Box<dyn MenuThing>) -> Self {
        Self {
            menu_things: vec![value],
            current: 0,
            screen_block: None,
        }
    }
}

impl DisplayManager {
    /// Uses `utils::block()` to set `self.screen_block`.
    ///
    /// Returns the previous value of `self.screen_block`.
    pub const fn default_screen_block(&mut self) -> Option<Block<'static>> {
        self.screen_block.replace(block())
    }

    /// Wrapper over `Self::default_screen_block()` that returns `Self`.
    #[must_use]
    pub const fn with_default_screen_block(mut self) -> (Self, Option<Block<'static>>) {
        let old_block: Option<Block<'static>> = self.screen_block.replace(block());
        (self, old_block)
    }

    /// Creates a new instance of `DisplayManager` with the new `MenuThing` as the selected one.
    pub fn from_menu_thing(menu_thing: Box<dyn MenuThing>) -> Self {
        Self::from(menu_thing)
    }

    /// Index of the currently displayed `MenuThing`.
    pub const fn current_index(&self) -> usize {
        self.current
    }

    /// Currently displayed `MenuThing`.
    ///
    /// # Panics
    /// If `self.current_index()` is an index higher than the length of `self.menu_things`.
    pub fn current_menu_thing(&self) -> &dyn MenuThing {
        self.menu_things[self.current_index()].as_ref()
    }

    /// Currently displayed `MenuThing` with mutable access.
    fn current_menu_thing_mut(&mut self) -> &mut dyn MenuThing {
        let current_index: usize = self.current_index();
        self.menu_things[current_index].as_mut()
    }

    /// Whether the next `MenuThing` exists.
    pub const fn has_next(&self) -> bool {
        self.current_index() != self.menu_things.len() - 1
    }

    /// Selects the next `MenuThing`, if one. If not, returns `false`.
    pub const fn select_next(&mut self) -> bool {
        if self.has_next() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Whether the previous `MenuThing` exists.
    pub const fn has_previous(&self) -> bool {
        self.current_index() != 0
    }

    /// Selects the previous `MenuThing`, if one. If not, returns `false`.
    pub const fn select_previous(&mut self) -> bool {
        if self.has_previous() {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Selects the first `MenuThing`.
    pub const fn select_first(&mut self) {
        self.current = 0;
    }

    /// Selects the last `MenuThing`.
    pub fn select_last(&mut self) {
        self.current = self.menu_things.len() - 1;
    }

    /// Adds a new `MenuThing` to the end of the list.
    pub fn add_menu_thing(
        &mut self,
        new: Box<dyn MenuThing>,
    ) {
        self.menu_things.push(new);
    }

    /// Adds and selects the new `MenuThing`.
    pub fn select_new(
        &mut self,
        new: Box<dyn MenuThing>,
    ) {
        self.add_menu_thing(new);
        self.select_last();
    }

    /// Whether the passed `MenuThing` was ever selected.
    pub fn has<MT: MenuThing>(&self) -> bool {
        let target: TypeId = TypeId::of::<MT>();
        #[expect(clippy::borrowed_box, reason = "no way to avoid it")]
        self.menu_things
            .iter()
            .any(|item: &Box<dyn MenuThing>| item.type_id() == target)
    }

    /// Select an already existing `MenuThing`.
    ///
    /// It will search the menu list in reverse chronological order and select the index of the
    /// first element it finds. In other words, it will go back in time, and the first time it
    /// encounters the wanted menu, it will select it.
    ///
    /// This will not move the target `MenuThing` to the front of the list.
    ///
    /// # Errors
    /// If `MT` was never selected.
    pub fn select_existing<MT: MenuThing>(&mut self) -> Result<()> {
        let target: TypeId = TypeId::of::<MT>();
        #[expect(clippy::borrowed_box, reason = "no way to avoid it")]
        {
            self.current = self
                .menu_things
                .iter()
                .rposition(|item: &Box<dyn MenuThing>| item.type_id() == target)
                .ok_or_eyre(format!("{} was never selected.", type_name::<MT>()))?;
        }
        // Ok.
        Ok(())
    }

    /// Selects the latest instance of `MT` (using `self.select_existing()`), if one is found.
    /// Otherwise, uses the constructor to get a new `MenuElement`, add it to the list, and select
    /// it (using `self.select_new()`).
    pub fn check_select<MT, F>(
        &mut self,
        constructor: F,
    ) where
        MT: MenuThing,
        F: FnOnce() -> Box<MT>,
    {
        if self.select_existing::<MT>().is_err() {
            self.select_new(constructor());
        }
    }

    /// Handles the passed `Event` by calling `MenuThing::handle_event()` on the current
    /// `MenuThing`.
    ///
    /// If the function's return value contains an `Action::DisplayManagerAction`, it will be
    /// handled and the return `Vec` will not include it.
    pub async fn handle_event(
        &mut self,
        app_event: AppEvent,
    ) -> Vec<Action> {
        self.current_menu_thing_mut()
            .handle_event(app_event)
            .await
            .into_iter()
            .filter_map(|action: Action| {
                if let Action::DisplayManagerAction(display_manager_action) = action {
                    self.handle_display_manager_action(display_manager_action);
                    None
                } else {
                    Some(action)
                }
            })
            .collect()
    }

    fn handle_display_manager_action(
        &mut self,
        action: DisplayManagerAction,
    ) {
        match action {
            DisplayManagerAction::NewMT(menu_thing) => self.add_menu_thing(menu_thing),
            DisplayManagerAction::SelectNewMT(menu_thing) => self.select_new(menu_thing),
            DisplayManagerAction::SelectPrevMT => {
                let _: bool = self.select_previous();
            }
            DisplayManagerAction::SelectNextMT => {
                let _: bool = self.select_next();
            }
        }
    }
}
