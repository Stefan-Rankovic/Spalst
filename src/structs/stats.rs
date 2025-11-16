/// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stats {
    strength: u64,
    speed: u64,
    //todo
}
