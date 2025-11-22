//! SPDX-License-Identifier: GPL-3.0-only

pub mod controls;
pub mod github;
pub mod paths;
pub mod style;
pub mod time;

pub use controls::{
    DOWN_KEYS, ENTER_KEYS, ESCAPE_KEYS, LEFT_KEYS, MULTIPLE_DOWN_KEYS, MULTIPLE_UP_KEYS,
    RIGHT_KEYS, UP_KEYS,
};
pub use github::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER};
pub use paths::{ENTITY_TEMPLATES_PATH, SPALST_SAVE_PATH};
pub use style::{BORDER_NOT_SELECTED, BORDER_SELECTED};
pub use time::{ACHIEVEMENT_DISPLAY_TIME, CREATE_PLAYTHROUGH_WARN_TIME};
