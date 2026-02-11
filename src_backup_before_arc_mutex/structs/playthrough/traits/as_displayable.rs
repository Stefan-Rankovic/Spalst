//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::style::{BORDER_NOT_SELECTED, BORDER_SELECTED},
    structs::{Duration, Playthrough},
    traits::AsDisplayable,
    types::ItemBlockInfo,
    utils::create_block,
};
use chrono::{DateTime, Utc};
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    widgets::Block,
};

impl<'n> AsDisplayable for Playthrough<'n> {
    fn as_displayable<'t, 'b>(&'b self, selected: bool) -> (Text<'t>, Option<ItemBlockInfo<'b>>) {
        // Define the content
        let content: Text = vec![
            Line::from(format!("Playtime: {}", self.playtime)),
            Line::from(format!("Number of saves: {}", self.saves.len())),
            //
            Line::from(format!(
                "Created: {}",
                Duration::try_from(Utc::now().signed_duration_since(self.created_at))
                    .unwrap()
                    .display_in_ago_format()
            )),
            Line::from(format!(
                "Last played: {}",
                self.last_played_at
                    .map(|dt: DateTime<Utc>| -> String {
                        Duration::try_from(Utc::now().signed_duration_since(dt))
                            .unwrap()
                            .display_in_ago_format()
                    })
                    .unwrap_or_else(|| "Never".to_string())
            )),
            //
            Line::from(format!(
                "Note: {}",
                self.note.as_ref().unwrap_or(&"None".to_string())
            ))
            .italic()
            .dim(),
        ]
        .into();
        // Create block
        let block: Block = create_block(Some(self.name), 1).border_type(if selected {
            BORDER_SELECTED
        } else {
            BORDER_NOT_SELECTED
        });
        // Ok.
        (content, Some((block, (4, 4))))
    }
}
