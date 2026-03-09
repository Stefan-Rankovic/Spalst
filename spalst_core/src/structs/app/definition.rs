//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{AchievementId, MenuElementsMainMenuSelected, MenuEvent},
    structs::{Account, AchievementQueue, Data, Menu, MenuElementsMainMenu},
    traits::{MenuElements as _, Saveable as _},
};
use color_eyre::eyre::{OptionExt as _, Result, eyre};
use core::{sync::atomic::AtomicBool, time::Duration};
use crossterm::event::{Event, EventStream};
use futures::StreamExt as _;
use rust_decimal_macros::dec;
use spalstatui::structs::{Frame, Terminal};
use std::{collections::HashMap, path::Path, sync::Arc};
use tokio::{
    sync::{Mutex, mpsc},
    time::timeout,
};

#[derive(Debug)]
pub struct App {
    /// The path to the executable.
    pub path: Arc<Path>,
    /// todo: update the program to use `AtomicBool`
    pub dev: Arc<AtomicBool>,

    pub menu: Menu,
    pub display_achievements_queue: AchievementQueue,

    /// Whether the program should prepare for an exit.
    /// The program will only exit if this is `true` and `exit_holds` is empty.
    pub prepare_exit: Arc<AtomicBool>,
    /// Holds all the reasons why the program can't exit yet. The key is the name of the process
    /// that's holding the exit, and the value is the reason.
    /// This will hold pairs even if `prepare_exit` is `false`.
    /// In practice, this means every thread will upload their status to this parameter at the
    /// beginning of their `while` loop, or beginning of their run if they don't have one. For
    /// those that do, they'll either submit a `const &str` and never update it, or regularly
    /// update it (if it changed) on every iteration of the while loop.
    /// uploads their reason at the beginning of t
    pub exit_holds: Arc<Mutex<HashMap<&'static str, String>>>,

    pub account: Account,
    pub data: Data,
}

impl App {
    #[expect(clippy::needless_pass_by_ref_mut, reason = "It's a todo.")]
    /// Handles the given event.
    pub async fn handle_event(&mut self, event: Event, terminal: &mut Terminal) -> Result<()> {
        if let Event::Resize(..) = event {
            self.display(terminal).await?;
        }

        match self.menu.handle_event(event).await {
            MenuEvent::Nothing => {}
            MenuEvent::SelectLeft(_amount) => todo!(),
            MenuEvent::SelectDown(_amount) => todo!(),
            MenuEvent::SelectUp(_amount) => todo!(),
            MenuEvent::SelectRight(_amount) => todo!(),
        }

        // Ok.
        Ok(())
    }

    /// todo: remove the display/ directory and move this to display.rs"
    pub async fn display(&self, terminal: &mut Terminal) -> Result<()> {
        terminal
            .draw(async |frame: &mut Frame<'_>| frame.render_widget(self, frame.area()).await)
            .await?;
        // Ok.
        Ok(())
    }

    pub async fn run(&mut self, mut terminal: Terminal) -> Result<()> {
        let (tx, mut rx): (mpsc::Sender<_>, mpsc::Receiver<_>) = mpsc::channel(100);

        // Asynchronous listener for events.
        // Dropping the JoinHandle<()> is fine because awaiting the Future is unnecessary.
        drop(tokio::spawn(async move {
            // Create a reader
            let mut reader: EventStream = EventStream::new();
            // If it's None then that means the stream ended
            while let Some(Ok(event)) = reader.next().await {
                // If the sending fails with an error, that means the receiever stopped listening,
                // so we can safely break the loop
                if tx.send(event).await.is_err() {
                    break;
                }
            }
        }));

        // Display the update menu
        self.menu = Menu::new(MenuElementsUpdate);
        self.update().await?;

        // Display the main menu
        self.menu = Menu::new(MenuElementsMainMenu::new(
            true,
            Some(MenuElementsMainMenuSelected::CreatePlaythrough),
            false, // todo: implement actual logic for the Last Played button.
            !self.account.playthroughs.is_empty(),
        ));
        // Main menu loop
        while let Some(main_menu) = self
            .menu
            .current()
            .as_any()
            .downcast_ref::<MenuElementsMainMenu>()
        {
            // Refresh the terminal
            self.display(&mut terminal).await?;
            // Check if the achievement queue should advance
            if self.display_achievements_queue.current().is_some()
                && self.display_achievements_queue.seconds_left()? <= dec!(0)
            {
                self.display_achievements_queue.finish_current();
            }
            // Do some things based on self.menu.current's selected element.
            #[expect(
                clippy::single_match,
                reason = "todo: this is temporary as idk where to put the commented out code"
            )]
            #[expect(
                clippy::wildcard_enum_match_arm,
                reason = "todo: this is temporary as idk where to put the commented out code"
            )]
            match *main_menu
                .selected_element()
                .ok_or_eyre(eyre!("No selected element in main menu."))?
            {
                // todo: move this code somewhere else
                // MenuElementsMainMenuSelected::CreatePlaythrough {
                //     current_input,
                //     warning_displayed_on,
                // } => {
                //     if let Some(instant) = warning_displayed_on
                //         && instant.elapsed().as_secs_f64() >= CREATE_PLAYTHROUGH_WARN_TIME
                //     {
                //         let current_input: String = current_input.to_string();
                //         self.menu_mut().set_same(MainMenuEnum::CreatePlaythrough {
                //             current_input,
                //             warning_displayed_on: None,
                //         })?;
                //     };
                // }
                MenuElementsMainMenuSelected::Quit => {
                    // Save.
                    self.account.save(&self.path.join("spalst_save")).await?;
                    // Ok.
                    return Ok(());
                }
                _ => {}
            };
            // Handle events at the end because that's the only thing that takes a mutable
            // reference to self and doesn't use the variable `menu`, and the mutable reference and
            // immutable reference can't exist at the same time.
            if let Ok(Some(event)) = timeout(
                Duration::from_millis((1000_u16 / self.account.fps()).into()),
                rx.recv(),
            )
            .await
            {
                // Handle events
                self.handle_event(event, &mut terminal).await?;
            }
        }
        self.account.award_achievement(
            AchievementId::EnterPlaythrough,
            &mut self.display_achievements_queue,
        );

        // todo: temp
        self.account.save(&self.path.join("spalst_save")).await?;
        todo!();

        // todo: uncomment this
        // while !self.exit {
        //     if let Ok(Some(event)) = timeout(Duration::from_millis(100), rx.recv()).await {
        //         self.handle_event(event, &mut terminal).await?;
        //     };
        // }

        // Before exiting, make sure to save.
        // self.account.save(&self.path.join("spalst_save"))?; // todo: uncomment this

        // Ok.
        // Ok(()) // todo: uncomment this
    }
}
