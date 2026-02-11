//! SPDX-License-Identifier: GPL-3.0-only

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::{any::Any, fmt::Debug};

pub trait MenuElement: Any + Debug + Sized + Widget {
    fn selectable(&self) -> bool;
    fn selected(&self) -> bool;

    fn as_any(&self) -> &(dyn Any + '_);
}
