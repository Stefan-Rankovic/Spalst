//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::MenuElementsManagePlaythrough, traits::MenuElements as _};
use core::pin::Pin;
use ratatui::{buffer::Buffer, layout::Rect};
use spalstatui::traits::WidgetRef;

impl WidgetRef for MenuElementsManagePlaythrough {
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
                current_element.render_with_block(*current_area, buf).await;
            }
        })
    }
}
