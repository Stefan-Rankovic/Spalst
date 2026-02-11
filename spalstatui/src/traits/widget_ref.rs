//! SPDX-License-Identifier: GPL-3.0-only

use crate::traits::Widget;
use core::pin::Pin;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait WidgetRef: Sync {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>>;
}

impl<W: WidgetRef> Widget for &W {
    async fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf).await;
    }
}
