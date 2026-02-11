//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::MenuElementsSortableList,
    traits::{AsDisplayable, MenuElements as _},
};
use core::{fmt::Debug, pin::Pin};
use ratatui::{buffer::Buffer, layout::Rect};
use spalstatui::traits::WidgetRef;

impl<
    ItemId: 'static + Copy + Debug + PartialEq + Send + Sync,
    Item: 'static + AsDisplayable + Debug + PartialEq + Send + Sync,
> WidgetRef for MenuElementsSortableList<ItemId, Item>
{
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            for (current_element, current_area) in self
                .elements()
                .await
                .iter()
                .zip(self.elements_area(area).iter())
            {
                current_element.render_ref(*current_area, buf).await;
            }
        })
    }
}
