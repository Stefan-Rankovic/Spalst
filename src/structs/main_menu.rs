//! SPDX-License-Identifier: GPL-3.0-only
use crate::enums::{MainMenuEnum, MainMenuEnumDiscriminants, Select};
use color_eyre::eyre::{OptionExt, Result, bail};
use std::mem::discriminant;
use strum::{IntoDiscriminant, IntoEnumIterator};

#[derive(Debug)]
pub struct MainMenu {
    current: MainMenuEnum,
    selected: Option<MainMenuEnumDiscriminants>,
}

impl From<MainMenuEnum> for MainMenu {
    fn from(enum_variant: MainMenuEnum) -> Self {
        let selected: Option<MainMenuEnumDiscriminants> =
            if enum_variant.discriminant() == MainMenuEnumDiscriminants::Browsing {
                Some(MainMenuEnumDiscriminants::selected_default())
            } else {
                None
            };
        Self {
            current: enum_variant,
            selected,
        }
    }
}

impl From<MainMenuEnumDiscriminants> for MainMenu {
    fn from(enum_variant: MainMenuEnumDiscriminants) -> Self {
        MainMenuEnum::from(enum_variant).into()
    }
}

impl MainMenu {
    /// Returns the value of `self.current`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Browsing);
    ///
    /// main_menu.set(MainMenuEnum::Settings);
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Settings);
    /// ```
    pub fn current(&self) -> &MainMenuEnum {
        &self.current
    }
    /// Returns the value of `self.selected`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    ///
    /// main_menu.set(MainMenuEnum::Settings);
    /// assert_eq!(main_menu.selected(), None);
    /// ```
    pub fn selected(&self) -> Option<MainMenuEnumDiscriminants> {
        self.selected
    }

    /// Sets `self.current` to `MainMenuEnum::Browsing` and `self.selected` to `Some(MainMenuEnumDiscriminants::selected_default())`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Quit);
    /// main_menu.browse();
    ///
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Browsing);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    /// ```
    pub fn browse(&mut self) {
        self.current = MainMenuEnum::Browsing;
        self.selected = Some(MainMenuEnumDiscriminants::selected_default());
    }
    /// Calls `self.browse()` and then returns `self`.
    /// It's important to understand that this returns `&mut MainMenu` and NOT `MainMenu`, and, as
    /// such, modifies the original value instead of consuming it.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Settings);
    ///
    /// if let Some(MainMenuEnumDiscriminants::CreatePlaythrough) = main_menu.browsing().selected() {
    ///     // Do something
    /// };
    ///
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Browsing);
    /// ```
    pub fn browsing(&mut self) -> &mut Self {
        self.browse();
        self
    }
    /// Sets `self.current` to another value. If the passed value is `MainMenuEnum::Browsing`, the
    /// program will run `warn!(...)` (because, just call `self.browse()` instead), and then call
    /// `self.browse()`.
    ///
    /// If `to` is the same variant of the enum `MainMenuEnum` as `self.current`, the function will
    /// run `warn!(...)` (because `self.set_same()` should've been called instead) and then call
    /// `self.set_same()`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants},
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnumDiscriminants::ManagePlaythroughs);
    /// main_menu.set(MainMenuEnum::Quit);
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Quit);
    ///
    /// main_menu.set(MainMenuEnum::Browsing); // This will also run warn!(...)
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Browsing);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    /// ```
    pub fn set<M>(&mut self, to: M)
    where
        M: Into<MainMenuEnum>,
    {
        let to: MainMenuEnum = to.into();
        if to.discriminant() == MainMenuEnumDiscriminants::Browsing {
            warn!(
                "Called function MainMenu::set() with the argument {}. It should always be preferred to call MainMenu::browse() instead.",
                MainMenuEnumDiscriminants::Browsing.as_str_debug()
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
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{
    ///         MainMenuEnum, MainMenuEnumDiscriminants,
    ///         ManagePlaythroughsMenu, ManagePlaythroughsSelected,
    ///         PlaythroughsSortBy
    ///     },
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::ManagePlaythroughs(
    ///     ManagePlaythroughsMenu::Select {
    ///         selected: ManagePlaythroughsSelected::SortBy,
    ///         sort_ascending: false,
    ///         sort_by: PlaythroughsSortBy::Name,
    ///     },
    /// ));
    /// main_menu.set_same(MainMenuEnum::ManagePlaythroughs(ManagePlaythroughsMenu::Select {
    ///     selected: ManagePlaythroughsSelected::SortAscending,
    ///     sort_ascending: false,
    ///     sort_by: PlaythroughsSortBy::CreatedAt,
    /// }));
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::ManagePlaythroughs);
    /// ```
    pub fn set_same<M>(&mut self, to: M) -> Result<()>
    where
        M: Into<MainMenuEnum>,
    {
        let to: MainMenuEnum = to.into();
        if self.current.discriminant() != to.discriminant() {
            bail!(
                "Can only call MainMenu::set_same() when the variants of self.current and the argument passed are the same."
            );
        };
        self.current = to;
        Ok(())
    }

    /// Changes `self.selected` but with the guarantee that the new `self.selected` will have the
    /// same variant as the old one.
    /// Depracated and useless because `self.selected` is now a discriminant, not a variant that
    /// can have values.
    ///
    /// # Errors
    /// Can only fail if `to` isn't the same variant as `self.selected`.
    ///
    /// # Examples
    /// ```
    /// use spalst::{
    ///     enums::{
    ///         MainMenuEnum, MainMenuEnumDiscriminants,
    ///         ManagePlaythroughsSelected, PlaythroughsSortBy
    ///     },
    ///     structs::MainMenu,
    /// };
    /// use strum::IntoDiscriminant;
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    /// main_menu.set_same(MainMenuEnumDiscriminants::Browsing);
    /// assert_eq!(main_menu.current().discriminant(), MainMenuEnumDiscriminants::Browsing);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    /// ```
    #[deprecated]
    pub fn select_same(&mut self, to: MainMenuEnumDiscriminants) -> Result<()> {
        let Some(ref mut selected) = self.selected else {
            bail!(
                "Can only call MainMenu::select when self.selected is Some and self.current is {}.",
                MainMenuEnumDiscriminants::Browsing.as_str_debug()
            );
        };
        if *selected != to {
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
    /// use spalst::{
    ///     enums::{MainMenuEnum, MainMenuEnumDiscriminants, Select},
    ///     structs::MainMenu
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::from(MainMenuEnum::Browsing);
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::Achievements));
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::Settings));
    ///
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::CreatePlaythrough));
    ///
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::Quit));
    ///
    /// main_menu.select(Select::Direct(MainMenuEnumDiscriminants::ManagePlaythroughs));
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnumDiscriminants::ManagePlaythroughs));
    /// ```
    pub fn select(&mut self, select: Select<MainMenuEnumDiscriminants>) -> Result<()> {
        let Some(ref mut selected) = self.selected else {
            bail!(
                "Can only call MainMenu::select when self.selected is Some and self.current is {}.",
                MainMenuEnumDiscriminants::Browsing.as_str_debug()
            );
        };
        // If the function should select directly
        if let Select::Direct(to_select) = select {
            if to_select == MainMenuEnumDiscriminants::Browsing {
                bail!("Tried to select MainMenuEnum::Browsing.");
            };
            if to_select == *selected {
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
        let variants: Vec<MainMenuEnumDiscriminants> = MainMenuEnumDiscriminants::iter()
            .filter(|variant: &MainMenuEnumDiscriminants| -> bool {
                *variant != MainMenuEnumDiscriminants::Browsing
            })
            .collect();
        let current_index: usize = variants
            .iter()
            .position(|variant: &MainMenuEnumDiscriminants| -> bool { *variant == *selected })
            .unwrap();
        match select {
            Select::Previous => {
                // Get the specific index of variants, clone it, and set selected to it. The index
                // is gotten by getting the current index, adding the length and then subtracting 1
                // (adding the length ensures that even if current_index is 0, subtracting 1 won't
                // trigger an overflow. And then from that number, the remainder of dividing by
                // variants.len() is gotten, which is the index needed.
                *selected = variants[(current_index + variants.len() - 1) % variants.len()]
            }
            Select::Next => *selected = variants[(current_index + 1) % variants.len()],
            Select::Direct(_) => unreachable!(),
        };
        // Ok.
        Ok(())
    }
}
