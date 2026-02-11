//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Duration, Save, SaveId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::Duration as DurationTokio;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playthrough<'n> {
    pub name: &'n str,

    pub playtime: Duration,
    pub saves: HashMap<SaveId, Save>,

    pub created_at: DateTime<Utc>,
    pub last_played_at: Option<DateTime<Utc>>,

    pub note: Option<String>,
}

impl<'n> Playthrough<'n> {
    fn new(name: &'n str) -> Self {
        Self {
            name,
            saves: HashMap::new(),
            note: None,
            created_at: Utc::now(),
            last_played_at: None,
            playtime: DurationTokio::ZERO.into(),
        }
    }
}
