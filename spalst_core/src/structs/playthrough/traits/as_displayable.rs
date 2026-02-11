//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::style::{BORDER_NOT_SELECTED, BORDER_SELECTED},
    structs::Playthrough,
    traits::AsDisplayable,
    types::ItemBlockInfo,
    utils::{display_duration, display_duration_ago_format},
};
use chrono::{DateTime, Utc};
use ratatui::{
    style::Stylize as _,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding},
};

impl AsDisplayable for Playthrough {
    fn as_displayable<'text, 'block>(
        &'block self,
        selected: bool,
    ) -> (Text<'text>, Option<ItemBlockInfo<'block>>) {
        // Define the content
        let content: Text<'text> = vec![
            Line::from(format!("Playtime: {}", display_duration(self.playtime))),
            Line::from(format!("Number of saves: {}", self.saves.len())),
            //
            Line::from(format!(
                "Created: {}",
                display_duration_ago_format((Utc::now() - self.created_at).to_std().unwrap())
            )),
            Line::from(format!(
                "Last played: {}",
                self.last_played_at.map_or_else(
                    || "Never".to_string(),
                    |dt: DateTime<Utc>| -> String {
                        display_duration_ago_format((Utc::now() - dt).to_std().unwrap())
                    }
                )
            )),
            //
            Line::from(format!("Note: {}", self.note.as_deref().unwrap_or("None")))
                .italic()
                .dim(),
        ]
        .into();
        // Create block
        let block: Block<'block> = Block::bordered()
            .border_set(border::ROUNDED)
            .padding(Padding::uniform(1))
            .title(self.name.clone())
            .border_type(if selected {
                BORDER_SELECTED
            } else {
                BORDER_NOT_SELECTED
            });
        // Ok.
        (content, Some((block, (4, 4))))
    }
}
