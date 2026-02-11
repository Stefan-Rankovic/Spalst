//! SPDX-License-Identifier: GPL-3.0-only

use std::{any::Any, marker::PhantomData};

use crate::traits::MenuElement;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Clone, Copy, Debug)]
pub struct MenuElementEmpty<'data> {
    _phantom: PhantomData<&'data ()>,
}

impl<'data> MenuElement for MenuElementEmpty<'data> {
    fn selectable(&self) -> bool {
        false
    }
    fn selected(&self) -> bool {
        false
    }

    fn as_any(&self) -> &(dyn Any + '_) {
        self
    }
}

impl<'data> Widget for MenuElementEmpty<'data> {
    #[allow(unused_variables)]
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
