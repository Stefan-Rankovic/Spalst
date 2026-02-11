//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{
    structs::{
        MenuElementEmpty, MenuElementKeyValue, MenuElementList, MenuElementRaw,
        MenuElementsMainMenu, MenuElementsManagePlaythrough, MenuElementsSortableList,
    },
    traits::MenuElement,
};

/// Enum that lists all MenuElements.
///
/// Because the struct `Menu` needs to be displayable, it needs `Widget`. To display it, the best
/// choice is `self.menu.current.render(area, buf)`. The problem with that is the fact that
/// `Widget::render()` requires `*self.current` to be a concrete type known at compile time (* at
/// the front because it's automatically dereferenced because `self.current` is a `Box`). But,
/// `self.current` is `Box<dyn MenuElement>`, which isn't a concrete type and changes at runtime.
/// That's impossible to circumvent. So, this enum was made to hold all possible values of
/// `self.current` so that there is actually a concrete type known at compile time.
/// todo: maybe rename this to MenuElement and the MenuElement trait to MenuElementTrait.
#[derive(Debug)]
pub enum MenuElementListEnum<'data> {
    // MenuElement
    MenuElementEmpty(MenuElementEmpty),
    MenuElementRaw(MenuElementRaw<'data>),
    MenuElementKeyValue(MenuElementKeyValue<'data>),
    MenuElementList(MenuElementList),

    // MenuElements - utility
    MenuElementsSortableList(MenuElementsSortableList),

    // MenuElements - true menus
    MenuElementsMainMenu(MenuElementsMainMenu),
    MenuElementsManagePlaythrough(MenuElementsManagePlaythrough),
}

impl<'data> From<MenuElement> for MenuElementListEnum<'data> {
    fn from(menu_element: impl MenuElement) -> Self {
        todo!()
    }
}

impl<'data> Widget for MenuElementListEnum<'data> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::MenuElementEmpty(element) => element.render(area, buf),
            Self::MenuElementRaw(element) => element.render(area, buf),
            Self::MenuElementKeyValue(element) => element.render(area, buf),
            Self::MenuElementList(element) => element.render(area, buf),

            Self::MenuElementsSortableList(element) => element.render(area, buf),

            Self::MenuElementsMainMenu(element) => element.render(area, buf),
            Self::MenuElementsManagePlaythrough(element) => element.render(area, buf),
        }
    }
}
