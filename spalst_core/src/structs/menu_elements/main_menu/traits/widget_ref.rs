//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::MenuElementsMainMenu, traits::MenuElements as _};
use core::pin::Pin;
use ratatui::{buffer::Buffer, layout::Rect};
use spalstatui::traits::WidgetRef;

impl WidgetRef for MenuElementsMainMenu {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move {
            for (element, element_area) in
                self.elements().await.iter().zip(self.elements_area(area))
            {
                element.render_ref(element_area, buf).await;
            }
        })
    }
}
