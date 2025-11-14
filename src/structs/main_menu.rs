use crate::enums::{MainMenuEnum, Select};
use color_eyre::eyre::{Result, bail};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct MainMenu {
    current: MainMenuEnum,
    selected: Option<MainMenuEnum>,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            current: MainMenuEnum::Browsing,
            selected: Some(MainMenuEnum::first()),
        }
    }
}

impl From<MainMenuEnum> for MainMenu {
    fn from(enum_variant: MainMenuEnum) -> MainMenu {
        let selected: Option<MainMenuEnum> = if enum_variant == MainMenuEnum::Browsing {
            Some(MainMenuEnum::first())
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
    /// Returns the current value of self.current.
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
    /// Returns the current value of self.selected.
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

    /// Sets self.current to MainMenuEnum::Browsing and self.selected to Some(MainMenuEnum::first()).
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
        self.selected = Some(MainMenuEnum::first());
    }
    /// todo
    pub fn browsing(&mut self) -> &mut Self {
        self.browse();
        self
    }
    /// Sets self.current to another value. If the passed value is MainMenuEnum::Browsing, the
    /// program will run warn!(...), and then just call self.browse().
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::MainMenuEnum,
    ///     structs::MainMenu,
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::default();
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
        } else {
            self.selected = None;
            self.current = to;
        };
    }

    /// Selects the previous or next item (changes `self.selected`).
    ///
    /// # Errors
    /// An error will only be returned if `self.selected` is None.
    ///
    /// # Examples
    /// ```
    /// # use assert_matches::assert_matches;
    /// use spalst::{
    ///     enums::{MainMenuEnum, Select},
    ///     structs::MainMenu
    /// };
    ///
    /// let mut main_menu: MainMenu = MainMenu::default();
    /// # assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    ///
    /// main_menu.select(Select::Next);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::LoadPlaythrough).as_ref());
    /// main_menu.select(Select::Next);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Achievements).as_ref());
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Settings).as_ref());
    /// main_menu.select(Select::Next);
    /// main_menu.select(Select::Next);
    /// assert_matches!(main_menu.selected(), Some(MainMenuEnum::CreatePlaythrough { .. }));
    /// main_menu.select(Select::Previous);
    /// assert_eq!(main_menu.selected(), Some(MainMenuEnum::Quit).as_ref());
    /// ```
    pub fn select(&mut self, select: Select) -> Result<()> {
        let Some(ref mut selected) = self.selected else {
            bail!(
                "Can only call MainMenu::select when self.selected is Some and self.current is {}.",
                MainMenuEnum::Browsing.as_str_debug()
            );
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
        };
        // Ok.
        Ok(())
    }
}
