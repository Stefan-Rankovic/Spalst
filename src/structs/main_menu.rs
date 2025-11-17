/// SPDX-License-Identifier: GPL-3.0-only
use crate::enums::{MainMenuEnum, Select};
use color_eyre::eyre::{OptionExt, Result, bail};
use std::mem::discriminant;
use strum::IntoEnumIterator;

#[derive(Debug, Eq, PartialEq)]
pub struct MainMenu {
    current: MainMenuEnum,
    selected: Option<MainMenuEnum>,
}

impl From<MainMenuEnum> for MainMenu {
    fn from(enum_variant: MainMenuEnum) -> MainMenu {
        let selected: Option<MainMenuEnum> = if enum_variant == MainMenuEnum::Browsing {
            Some(MainMenuEnum::selected_default())
        } else {
            None
        };
        Self {
            current: enum_variant,
            selected,
        }
    }
}

impl MainMenu {
    /// Returns the value of `self.current`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    ///
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Browsing);
    ///
    /// main_menu.set(MainMenuEnum::Settings);
    ///
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Settings);
    /// ```
    pub fn current(&self) -> &MainMenuEnum {
        &self.current
    }
    /// Returns the value of `self.selected`.
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    ///
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    ///
    /// main_menu.set(MainMenuEnum::Settings);
    ///
    /// assert_eq!(main_menu.selected(), None);
    /// ```
    pub fn selected(&self) -> Option<&MainMenuEnum> {
        self.selected.as_ref()
    }

    /// Sets `self.current` to `MainMenuEnum::Browsing` and `self.selected` to `Some(MainMenuEnum::selected_default())`.
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Quit);
    ///
    /// main_menu.browse();
    ///
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Browsing);
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    /// ```
    pub fn browse(&mut self) {
        self.current = MainMenuEnum::Browsing;
        self.selected = Some(MainMenuEnum::selected_default());
    }
    /// Calls `self.browse()` and then returns `self`.
    /// It's important to understand that this returns `&mut MainMenu` and NOT `MainMenu`, and, as
    /// such, modifies the original value instead of consuming it.
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Settings);
    ///
    /// if let Some(MainMenuEnum::CreatePlaythrough {
    ///     current_input,
    ///     warning_displayed_on,
    /// }) = main_menu.browsing().selected() {
    ///     // Do something
    /// };
    ///
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Browsing);
    ///
    /// let mut main_menu_other: MainMenu = MainMenu::from(MainMenuEnum::LoadPlaythrough);
    ///
    /// main_menu_other.browse();
    ///
    /// # assert_eq!(main_menu, main_menu_other);
    /// ```
    pub fn browsing(&mut self) -> &mut Self {
        self.browse();
        self
    }
    /// Sets `self.current` to another value. If the passed value is `MainMenuEnum::Browsing`, the
    /// program will run `warn!(...)` (beacuse, just call `self.browse()` instead), and then call
    /// `self.browse()`.
    ///
    /// If `to` is the same variant of the enum `MainMenuEnum` as `self.current`, the function will
    /// run `warn!(...)` (because `self.set_same()` should've been called instead) and then call
    /// `self.set_same()`.
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::LoadPlaythrough);
    /// main_menu.set(MainMenuEnum::Quit);
    ///
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Quit);
    ///
    /// main_menu.set(MainMenuEnum::Browsing); // This will also run warn!()
    ///
    /// assert_eq!(*main_menu.current(), MainMenuEnum::Browsing);
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    /// ```
    pub fn set(&mut self, to: MainMenuEnum) {
        if to == MainMenuEnum::Browsing {
            warn!(
                "Called function MainMenu::set() with the argument {}. It should always be preferred to call MainMenu::browse() instead.",
                MainMenuEnum::Browsing.as_str_debug()
            );
            self.browse()
        } else if discriminant(&to) == discriminant(&self.current) {
            warn!(
                "Called function MainMenu::set() with the argument {:#?}, which is the same variant as the current value, {:#?}. It should always be preferred to call MainMenu::set_same() instead.",
                to, self.current,
            );
            self.set_same(to)
                .expect("Only condition for fail checked 2 lines above.");
        } else {
            self.selected = None;
            self.current = to;
        };
    }

    /// Changes `self.current` but with the guarantee that the new `self.current` will have the
    /// same variant as the old one.
    ///
    /// # Errors
    /// Can only fail if `to` isn't the same variant as `self.current`.
    pub fn set_same(&mut self, to: MainMenuEnum) -> Result<()> {
        if discriminant(&self.current) != discriminant(&to) {
            bail!(
                "Can only call MainMenu::set_same() when the variants of self.current and the argument passed are the same."
            );
        };
        self.current = to;
        Ok(())
    }

    /// Changes `self.selected` but with the guarantee that the new `self.selected` will have the
    /// same variant as the old one.
    pub fn select_same(&mut self, to: MainMenuEnum) -> Result<()> {
        let Some(ref mut selected) = self.selected else {
            bail!(
                "Can only call MainMenu::select when self.selected is Some and self.current is {}.",
                MainMenuEnum::Browsing.as_str_debug()
            );
        };
        if discriminant(selected) != discriminant(&to) {
            bail!(
                "Can only call MainMenu::select_same() when the variants of self.selected and the argument passed are the same."
            );
        } else {
            *selected = to;
        };
        // Ok.
        Ok(())
    }

    /// Changes `self.selected` according to the value of `select`.
    ///
    /// If `select` is `Select::Direct(value)` and `value` is the same variant of MainMenuEnum as
    /// `self.selected`, the function will run `warn!(...)` (beacuse `self.select_same()` should've
    /// been called instead) and then call `self.select_same()`.
    ///
    /// # Errors
    /// An error will be returned if any of these are true:
    ///     - `self.selected` is `None`.
    ///     - `select` is `Select::Direct(MainMenuEnum::Browsing)`
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::{MainMenuEnum, Select},
    ///     structs::MainMenu
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Achievements).as_ref());
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Settings).as_ref());
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    ///
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Quit).as_ref());
    ///
    /// main_menu.select(Select::Direct(MainMenuEnum::LoadPlaythrough));
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::LoadPlaythrough).as_ref());
    /// ```
    pub fn select(&mut self, select: Select) -> Result<()> {
        let Some(ref mut selected) = self.selected else {
            bail!(
                "Can only call MainMenu::select when self.selected is Some and self.current is {}.",
                MainMenuEnum::Browsing.as_str_debug()
            );
        };

        if let Select::Direct(to_select) = select {
            if to_select == MainMenuEnum::Browsing {
                bail!("Tried to select MainMenuEnum::Browsing.");
            };
            if discriminant(&to_select) == discriminant(selected) {
                warn!(
                    "Called function MainMenu::select() with the argument {:#?}, which is the same variant as the current value, {:#?}. It should always be preferred to call MainMenu::select_same() instead.",
                    to_select, selected,
                );
                self.select_same(to_select)?;
                // Ok.
                return Ok(());
            };
            *selected = to_select;
            // Ok.
            return Ok(());
        };
        let variants: Vec<MainMenuEnum> = MainMenuEnum::iter()
            .filter(|variant| *variant != MainMenuEnum::Browsing)
            .collect();
        let current_index: usize = variants
            .iter()
            .position(|variant| std::mem::discriminant(variant) == std::mem::discriminant(selected))
            .unwrap();
        match select {
            Select::Previous => {
                // Get the specific index of variants, clone it, and set selected to it. The index
                // is gotten by getting the current index, adding the length and then subtracting 1
                // (adding the length ensures that even if current_index is 0, subtracting 1 won't
                // trigger an overflow. And then from that number, the remainder of dividing by
                // variants.len() is gotten, which is the index needed.
                *selected = variants[(current_index + variants.len() - 1) % variants.len()].clone()
            }
            Select::Next => *selected = variants[(current_index + 1) % variants.len()].clone(),
            Select::Direct(_) => unreachable!(),
        };
        // Ok.
        Ok(())
    }
}
