/// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    name: String,
    usable: bool,
    wearable: bool, // todo: expand this to wearable, but where. its not the same if its wearable
                    // on your head or torso
}
