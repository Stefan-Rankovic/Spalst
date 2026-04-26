//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{SafetyLevel, Updater};

// impl Updater {
//     pub async fn exit_early(&self) {
//         let safety_level_of_current: SafetyLevel = self
//             .releases
//             .lock()
//             .await
//             .safety_level_of_current();
//
//         if safety_level_of_current.is_safe() { return; }
//
//         if safety_level_of_current.exit_update_early() {
//             panic!("The current version is unsafe for updating.");
//         }
//     }
// }
