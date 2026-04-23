//! SPDX-License-Identifier: GPL-3.0-only

use std::{
    env,
    path::{Path, PathBuf},
};

/// The parent of all top-level files and directories of the game including the game executable
/// itself.
///
/// Accounts for a development environment (where the parent is target/debug/).
pub fn game_directory() -> PathBuf {
    #[expect(
        clippy::expect_used,
        reason = "These errors will never actually happen anyway."
    )]
    let parent_of_executable: PathBuf = env::current_exe()
        .expect("The executable location of the game can't be found. Honestly this is rare enough for a cookie 🍪")
        .parent()
        .expect("The game executable doesn't have a parent. Please give it one. How did you even manage to do this? Have a cookie for your efforts 🍪")
        .to_path_buf();

    // If the parent is ".../target/debug/", that means it's (probably) in a development
    // environment, in which case the game files are not actually in "target/debug/".
    if parent_of_executable.ends_with("target/debug")
        && let Some(Some(parent_of_target)) = parent_of_executable.parent().map(Path::parent)
    {
        parent_of_target.to_path_buf()
    } else {
        parent_of_executable
    }
}
