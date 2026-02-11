//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Save, SaveId};
use chrono::{DateTime, Utc};
use core::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playthrough {
    pub name: String,

    pub playtime: Duration,
    pub saves: HashMap<SaveId, Save>,

    pub created_at: DateTime<Utc>,
    pub last_played_at: Option<DateTime<Utc>>,

    pub note: Option<String>,
}

impl Playthrough {
    pub fn new(name: String) -> Self {
        Self {
            name,
            saves: HashMap::new(),
            note: None,
            created_at: Utc::now(),
            last_played_at: None,
            playtime: Duration::ZERO,
        }
    }
}
