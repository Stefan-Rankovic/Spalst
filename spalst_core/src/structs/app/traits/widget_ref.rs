//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::App;
use core::pin::Pin;
use ratatui::{buffer::Buffer, layout::Rect};
use spalstatui::traits::WidgetRef;

impl WidgetRef for App {
    fn render_ref<'future>(
        &'future self,
        area: Rect,
        buf: &'future mut Buffer,
    ) -> Pin<Box<dyn Future<Output = ()> + 'future + Send>> {
        Box::pin(async move { self.menu.render_ref(area, buf).await })
        // todo: move this somewhere else
        // Display achievements gotten
        // if let Some(achievement) = self.display_achievements_queue.current() {
        //     let (paragraph, popup_area) = create_popup(
        //         inner_area,
        //         VerticalAlignment::Top,
        //         Alignment::Right,
        //         12,
        //         20,
        //         Some("Notification"),
        //         vec![
        //             Line::from(format!(
        //                 "Acquired achievement \"{}\"!",
        //                 achievement.name_user()
        //             )),
        //             Line::from(""),
        //             Line::from(format!(
        //                 "{:.1}",
        //                 self.display_achievements_queue.seconds_left().unwrap(),
        //             )),
        //         ],
        //     );
        //     paragraph
        //         .alignment(Alignment::Center)
        //         .render(popup_area, buf);
    }
}
