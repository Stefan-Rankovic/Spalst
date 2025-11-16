/// SPDX-License-Identifier: GPL-3.0-only
use crate::structs::{Item, Stats};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    name: String,
    race: String,
    inventory: Vec<Item>,
    stats: Stats,
}
