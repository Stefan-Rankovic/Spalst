//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{BORDER_NOT_SELECTED, BORDER_SELECTED},
    structs::{PlaythroughName, Save, SaveId},
    utils::{create_block, time_delta_format},
};
use chrono::{DateTime, Utc};
use ratatui::{
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playthrough {
    pub saves: HashMap<SaveId, Save>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_played_at: Option<DateTime<Utc>>,
    pub playtime: Duration,
}

impl Default for Playthrough {
    fn default() -> Self {
        Self {
            saves: HashMap::new(),
            note: None,
            created_at: Utc::now(),
            last_played_at: None,
            playtime: Duration::ZERO,
        }
    }
}

impl Playthrough {
    pub fn as_displayable<'a>(
        &'a self,
        selected: bool,
        name: &PlaythroughName,
    ) -> (Text<'a>, Block<'a>) {
        // Define the content
        let content: Text = vec![
            Line::from(format!("Number of saves: {}", self.saves.len())),
            Line::from(format!(
                "Last played: {}",
                self.last_played_at
                    .map(|dt: DateTime<Utc>| -> String {
                        time_delta_format(Utc::now().signed_duration_since(dt))
                    })
                    .unwrap_or_else(|| "Never".to_string())
            )),
            Line::from(format!(
                "Created: {}",
                time_delta_format(Utc::now().signed_duration_since(self.created_at))
            )),
            Line::from(format!(
                "Note: {}",
                self.note.as_ref().unwrap_or(&"None".to_string())
            ))
            .italic()
            .dim(),
        ]
        .into();
        // Create block
        let block: Block = create_block(Some(name.0.clone()), 1).border_type(if selected {
            BORDER_SELECTED
        } else {
            BORDER_NOT_SELECTED
        });
        // Ok.
        (content, block)
    }
}
