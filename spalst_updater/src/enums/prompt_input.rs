//! SPDX-License-Identifier: GPL-3.0-only

use octocrab::models::repos::Release;

#[derive(Clone, Debug, Default)]
pub enum PromptInput {
    #[default]
    NotAwaiting,
    Awaiting,
    Update {
        to: Box<Release>,
    },
    DontUpdate,
}
