//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::SPALST_SAVE_PATH,
    structs::{Account, AchievementQueue, App, Data, Menu, MenuElementEmpty},
    traits::LoadableSafe as _,
};
use color_eyre::eyre::{ContextCompat as _, Result, bail};
use core::sync::atomic::{AtomicBool, Ordering};
use log::{error, warn};
use std::{collections::HashMap, env, path::Path, sync::Arc};
use tokio::sync::Mutex;

impl App {
    pub async fn try_new() -> Result<Self> {
        // The path of the parent of the executable.
        let path: Arc<Path> = env::current_exe()?.parent().wrap_err_with(|| "Your executable has no parent directory. Congrats. Now stop being a bumfuzzle and don't torture your env, nor the game.")?.into();

        // Whether the program is in a development environment.
        let dev: Arc<AtomicBool> = Arc::new(match env::var("CARGO_MANIFEST_PATH") {
            Ok(_) => true,
            Err(error) => match error {
                env::VarError::NotPresent => false,
                env::VarError::NotUnicode(_) => {
                    warn!(
                        "The CARGO_MANIFEST_PATH environment variable contains non-unicode data."
                    );
                    true
                }
            },
        }
        .into());
        if dev.load(Ordering::Acquire) {
            if path.ends_with("target/release") {
                dev.store(false, Ordering::Release);
            } else if !path.ends_with("target/debug") {
                error!(
                    "Developer mode was set and yet the parent directories aren't \"target/debug\" nor \"target/release\"."
                );
                bail!(
                    "Your executable isn't in the correct development path yet your CARGO_MANIFEST_PATH environment variable was set. Please unset it and then run the program."
                );
            } else {
                // Path ends with target/debug, which doesn't matter.
            }
        }

        let save_path: &Path = &path.join(SPALST_SAVE_PATH);
        let account: Account = Account::load_safe(save_path).await?;
        let display_achievements_queue: AchievementQueue = AchievementQueue::default();
        Ok(Self {
            path,
            dev,

            menu: Menu::new(MenuElementEmpty::default()),
            display_achievements_queue,

            prepare_exit: Arc::new(AtomicBool::new(false)),
            exit_holds: Arc::new(Mutex::new(HashMap::new())),

            account,
            data: Data,
        })
    }
}
