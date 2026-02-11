//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::MenuEvent, traits::MenuElement};
use color_eyre::eyre::{OptionExt as _, Result};
use core::{
    any::{TypeId, type_name},
    pin::Pin,
};
use crossterm::event::Event;
use ratatui::{buffer::Buffer, layout::Rect};
use spalstatui::traits::WidgetRef;

/// A menu manager. Manages the current menu and the history of previous menus.
/// Always guaranteed to have at least one menu.
/// Menu deletion isn't possible.
#[derive(Debug)]
pub struct Menu {
    /// History of menus, first element is the oldest menu.
    menus: Vec<Box<dyn MenuElement>>,
    /// Currently selected menu (index based).
    selected: usize,
    // pub current: MenuElementListEnum,
}

impl WidgetRef for Menu {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        self.current().render_with_block(area, buf)
    }
}

impl Menu {
    pub fn new<ME: MenuElement>(menu_element: ME) -> Self {
        Self {
            menus: vec![Box::new(menu_element)],
            selected: 0,
        }
    }
}

impl Menu {
    /// Index of the currently selected menu.
    pub const fn current_index(&self) -> usize {
        self.selected
    }
    /// Current selected menu.
    ///
    /// # Panics
    /// If `self.current_index()` is an index higher than the length of `self.menus` allows.
    pub fn current(&self) -> &dyn MenuElement {
        self.menus[self.selected].as_ref()
    }

    /// Whether the next menu exists.
    pub fn has_next(&self) -> bool {
        self.selected != self.menus.len() - 1
    }
    /// Select the next menu, if one. If not, returns `false`.
    pub fn select_next(&mut self) -> bool {
        if self.has_next() {
            self.selected += 1;
            true
        } else {
            false
        }
    }
    /// Whether the previous menu exists.
    pub const fn has_previous(&self) -> bool {
        self.selected != 0
    }
    /// Select the previous menu, if one. If not, returns `false`.
    pub const fn select_previous(&mut self) -> bool {
        if self.has_previous() {
            self.selected -= 1;
            true
        } else {
            false
        }
    }

    /// Selects the first menu.
    pub const fn select_first(&mut self) {
        self.selected = 0;
    }
    /// Selects the last menu.
    pub fn select_last(&mut self) {
        self.selected = self.menus.len() - 1;
    }

    /// Adds a new menu to the end of the list.
    pub fn add_menu<ME: MenuElement>(&mut self, new: ME) {
        self.menus.push(Box::new(new));
    }
    /// Adds and selects the new menu.
    pub fn select_new<ME: MenuElement>(&mut self, new: ME) {
        self.add_menu(new);
        self.select_last();
    }

    /// Whether the target menu was ever selected.
    pub fn has<ME: MenuElement>(&self) -> bool {
        let target: TypeId = TypeId::of::<ME>();
        #[expect(clippy::borrowed_box, reason = "no way to avoid it")]
        self.menus
            .iter()
            .any(|item: &Box<dyn MenuElement>| item.as_any().type_id() == target)
    }
    /// Select an already existing menu.
    ///
    /// It will search the menu list in reverse chronological order and select the index of the
    /// first element it finds. In other words, it will go back in time, and the first time it
    /// encounters the wanted menu, it will select it.
    ///
    /// # Errors
    /// If `ME` was never selected.
    pub fn select_existing<ME: MenuElement>(&mut self) -> Result<()> {
        let target: TypeId = TypeId::of::<ME>();
        #[expect(clippy::borrowed_box, reason = "no way to avoid it")]
        {
            self.selected = self
                .menus
                .iter()
                .rposition(|item: &Box<dyn MenuElement>| item.as_any().type_id() == target)
                .ok_or_eyre(format!("{} was never selected.", type_name::<ME>()))?;
        }
        // Ok.
        Ok(())
    }

    /// Selects the latest instance of `ME` if one (using `self.select_existing()`), if not use the
    /// constructor to construct a new `MenuElement`, add it to the list, and select it (using
    /// `self.select_new()`).
    pub fn check_select<ME, F>(&mut self, constructor: F)
    where
        ME: MenuElement,
        F: FnOnce() -> ME,
    {
        if self.select_existing::<ME>().is_err() {
            self.select_new(constructor());
        }
    }

    /// Handles the passed event.
    pub async fn handle_event(&self, event: Event) -> MenuEvent {
        match event {
            Event::Key(key_event) => self.current().handle_key_event(key_event).await,
            Event::Resize(..) => unreachable!(),
            Event::FocusLost | Event::FocusGained | Event::Mouse(..) | Event::Paste(..) => {
                MenuEvent::Nothing
            }
        }
    }
}
