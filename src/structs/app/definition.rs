//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::{CREATE_PLAYTHROUGH_WARN_TIME, SPALST_SAVE_PATH},
    enums::{
        AchievementId, MainMenuEnum, ManagePlaythroughsMenu, ManagePlaythroughsSelected,
        PlaythroughsSortBy,
    },
    structs::{Account, AchievementQueue, Data, Game, MainMenu},
    traits::{LoadableSafe, Saveable},
    utils::create_block,
};
use color_eyre::eyre::{Context, ContextCompat, Result, bail};
use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::DefaultTerminal;
use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{sync::mpsc, time::timeout};

#[derive(Debug)]
pub struct App {
    path: PathBuf,
    dev: bool,

    menu: Option<MainMenu>,
    pub display_achievements_queue: AchievementQueue,

    exit: bool,

    pub account: Account,
    data: Data,
}

impl App {
    pub async fn try_new() -> Result<Self> {
        let mut dev: bool = match env::var("CARGO_MANIFEST_PATH") {
            Ok(_) => true,
            Err(e) => match e {
                VarError::NotPresent => false,
                VarError::NotUnicode(_) => {
                    warn!(
                        "The CARGO_MANIFEST_PATH environment variable contains non-unicode data."
                    );
                    true
                }
            },
        };
        let path: PathBuf = {
            let mut p: PathBuf = std::env::current_exe()?.parent().wrap_err_with(|| "Your executable has no parent directory. Congrats. Now stop being a bumfuzzle and don't torture your env, nor the game.")?.to_path_buf();
            if dev {
                if p.ends_with("target/release") {
                    dev = false;
                };
                if p.ends_with("target/debug") || p.ends_with("target/release") {
                    p = p
                        .parent()
                        .wrap_err_with(|| "Your parents of the executable were found to be something and then \"debug\" (or \"release\") and then \"target\" and yet your executable doesn't have the 2nd parent.")?
                        .parent()
                        .wrap_err_with(|| "Your parents of the executable were found to be something and then \"debug\" (or \"release\") and then \"target\" and yet your executable doesn't have the 3rd parent.")?
                        .to_path_buf();
                } else {
                    error!(
                        "Developer mode was set and yet the parent directories aren't \"target/debug\" nor \"target/release\"."
                    );
                    bail!(
                        "You are not a developer yet your CARGO_MANIFEST_PATH environment variable was set. Please unset it and then run the program."
                    );
                };
            };
            p
        };
        let save_path: &Path = &path.join(SPALST_SAVE_PATH);
        let account = Account::load_safe(save_path);
        let data = Data::try_new(&path);
        let (account_result, data_result): (Result<Account>, Result<Data>) =
            tokio::join!(account, data);
        let account: Account = account_result.wrap_err_with(|| "Tried loading Account.")?;
        let data: Data = data_result?;
        let display_achievements_queue: AchievementQueue = AchievementQueue::default();
        Ok(Self {
            path: path.clone(),
            menu: None,
            display_achievements_queue,
            exit: false,
            account,
            data,
            dev,
        })
    }
}

impl App {
    /// Convenience method. Returns the output of "self.menu.as_ref().unwrap()".
    pub fn menu(&self) -> &MainMenu {
        self.menu.as_ref().unwrap()
    }
    /// Convenience method. Returns the output of "self.menu.as_mut().unwrap()".
    pub fn menu_mut(&mut self) -> &mut MainMenu {
        self.menu.as_mut().unwrap()
    }
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let (tx, mut rx): (mpsc::Sender<_>, mpsc::Receiver<_>) = mpsc::channel(100);

        // Asynchronous istener for events
        tokio::spawn(async move {
            // Create a reader
            let mut reader: EventStream = EventStream::new();
            // If it's None then that means the stream ended
            while let Some(Ok(event)) = reader.next().await {
                // If the sending fails with an error, that means the receiever stopped listening,
                // so we can safely break the loop
                if tx.send(event).await.is_err() {
                    break;
                };
            }
        });

        // Initialize the game using the Account
        self.account.initialize_game(&self.path)?;
        // Display the main menu
        self.menu = Some(MainMenu::from(MainMenuEnum::Browsing));
        // Main menu loop
        while let Some(menu) = self.menu.as_ref() {
            // Refresh the terminal
            self.display(&mut terminal)?;
            // Check if the achievement queue should advance
            if self.display_achievements_queue.current().is_some()
                && self.display_achievements_queue.seconds_left()? < 0.0
            {
                self.display_achievements_queue.finish_current();
            };
            // Do some things based on self.menu.current().
            match menu.current() {
                MainMenuEnum::CreatePlaythrough {
                    current_input,
                    warning_displayed_on,
                } => {
                    if let Some(instant) = warning_displayed_on
                        && instant.elapsed().as_secs_f64() >= CREATE_PLAYTHROUGH_WARN_TIME
                    {
                        let current_input: String = current_input.to_string();
                        self.menu_mut().set_same(MainMenuEnum::CreatePlaythrough {
                            current_input,
                            warning_displayed_on: None,
                        })?;
                    };
                }
                MainMenuEnum::Quit => {
                    // Save.
                    self.account.save(&self.path.join("spalst_save"))?;
                    // Ok.
                    return Ok(());
                }
                _ => {}
            };
            // Handle events at the end because that's the only thing that takes a mutable
            // reference to self and doesn't use the variable menu, and the mutable reference and
            // immutable reference can't exist at the same time.
            if let Ok(Some(event)) = timeout(
                Duration::from_millis((1000u16 / self.account.fps()).into()),
                rx.recv(),
            )
            .await
            {
                // Handle events
                self.handle_event(event, &mut terminal)?;
            };
        }
        self.account.award_achievement(
            AchievementId::EnterPlaythrough,
            &mut self.display_achievements_queue,
        );

        // todo: temp
        self.account.save(&self.path.join("spalst_save"))?;
        todo!();

        while !self.exit {
            if let Ok(Some(event)) = timeout(Duration::from_millis(100), rx.recv()).await {
                self.handle_event(event, &mut terminal)?;
            };
        }

        // Before exiting, make sure to save.
        self.account.save(&self.path.join("spalst_save"))?;
        Ok(())
    }
}
