use crate::{
    consts::SPALST_SAVE_PATH,
    enums::MainMenuEnum,
    structs::{Account, Data, Game, MainMenu},
    traits::{LoadableSafe, Saveable},
    utils::create_block,
};
use color_eyre::eyre::{Context, Result};
use crossterm::event::EventStream;
use futures::StreamExt;
use ratatui::DefaultTerminal;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::{sync::mpsc, time::timeout};

#[derive(Debug)]
pub struct App {
    path: PathBuf,
    pub menu: Option<MainMenu>,
    exit: bool,
    pub account: Account,
    data: Data,
    pub dev: bool,
}

impl App {
    pub fn try_new() -> Result<Self> {
        let path: PathBuf = std::env::current_exe()?.parent().unwrap().to_path_buf();
        Ok(Self {
            path: path.clone(),
            menu: None,
            exit: false,
            account: Account::load_safe(&path.join(SPALST_SAVE_PATH))
                .wrap_err_with(|| "Tried loading Account.")?,
            data: Data::try_new(&path)?,
            dev: false,
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
        self.display(&mut terminal)?;
        while self.menu.is_some() {
            // Uncomment the following line if, in the future, there exists an async task that can
            // change the value of self.menu. Currently, self.menu can only ever change from inside
            // this while loop, but in the case where it can change from other causes (such as an
            // async task changing it), rx.recv().await would block this thread until an event is
            // received, and the while loop condition won't get checked. So even though self.menu
            // isn't Menu::ChoosingSave, the loop will still run. Until an event, of course. That
            // can be prevented by timeouting every 100ms, which basically means the while loop
            // gets to check its condition every 100ms to see if self.menu magically changed.
            //if let Ok(Some(event)) = tokio::time::timeout(Duration::from_millis(100), rx.recv()).await {
            if let Some(event) = rx.recv().await {
                self.handle_event(event, &mut terminal)?;
                // Refresh the terminal
                self.display(&mut terminal)?;
                // Check if the program should quit
                if *self.menu().current() == MainMenuEnum::Quit {
                    self.account.save(&self.path.join("spalst_save"))?;
                    return Ok(());
                };
            };
        }

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
