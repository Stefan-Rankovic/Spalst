use crate::structs::{Save, SaveId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Playthrough {
    pub saves: HashMap<SaveId, Save>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_played_at: Option<DateTime<Utc>>,
}

impl Default for Playthrough {
    fn default() -> Self {
        Self {
            saves: HashMap::new(),
            note: None,
            created_at: Utc::now(),
            last_played_at: None,
        }
    }
}
