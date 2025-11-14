use crate::enums::MainMenu;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Menu {
    None,
    LoadingAccount,
    Main(MainMenu),
    GameMenu,
}

impl Default for Menu {
    fn default() -> Self {
        Self::None
    }
}
