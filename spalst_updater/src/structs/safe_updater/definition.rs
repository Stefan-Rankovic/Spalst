//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Releases;
use derive_new::new;

/// The thing that updates the program.
///
/// The current release will always be safe.
#[derive(Debug, new)]
pub struct SafeUpdater {
    // /// The current phase of the update process. See `Phase` documentation for more information.
    // pub(super) phase: Arc<Mutex<Phase>>, // todo: maybe remove this
    /// The list of all Github releases. See `Releases` documentation for more information.
    pub(super) releases: Releases,
    // /// Whether the struct is already doing something.
    // ///
    // /// Useful to prevent two threads trying to update at the same time.
    // pub(super) running: Arc<AtomicBool>, // todo: maybe remove as this is now enforced by the type system (updating takes `self`, not `&self`)

    // pub(super) prompt_input: Arc<Mutex<PromptInput>>, // todo: maybe remove this
    // /// When `prompt_input` is changed from `Awaiting`, this notifies all waiters.
    // pub(super) stopped_waiting_notifier: Arc<Notify>, // todo: maybe remove this
}

// impl SafeUpdater {
//     /// Spawns a task that progresses through the current phase.
//     ///
//     /// The function will not update the phase. That is left to subfunctions or the caller.
//     ///
//     /// # Errors
//     /// If another update process is already executing.
//     pub fn execute_phase(self: Arc<Self>) -> Result<JoinHandle<Result<()>>> {
//         let this: Arc<Self> = Arc::clone(&self);
//
//         Ok(tokio::spawn(async move {
//             match *self.phase.lock().await {
//                 Phase::CheckingForUpdates => this.check_for_updates().await,
//                 Phase::UpdatingFromUnsafe => this.update_to_safe().await,
//                 Phase::UpdatingToLatest => this.update_to_latest().await,
//                 Phase::Updating { ref to } => this.update_to(to).await,
//                 Phase::ShowingPrompt | Phase::Updated | Phase::DidNotUpdate | Phase::EarlyUnsafeUpdateExit => Ok(()),
//                 Phase::EarlyUnsafeProgramExit => Self::early_program_exit(),
//             }
//         }))
//     }
//
//     /// Early exit from the program with a panic message.
//     ///
//     /// Used to handle the case `EarlyUnsafeProgramExit`.
//     fn early_program_exit() -> ! {
//         panic!("The update phase was Phase::EarlyUnsafeProgramExit. No more info is attached since the program isn't in a normal state. Please check the GitHub page for more details.");
//     }
//
//     /// Check if the current version is the latest one.
//     ///
//     /// Modifies `self.phase` accordingly.
//     ///
//     /// # Panics
//     /// If the current version is newer than the latest version.
//     pub async fn check_for_updates(self: Arc<Self>) -> Result<()> {
//         self.start_doing_something()?;
//
//         let latest_version: Version = self.releases.lock().await.latest_version();
//         let current_version: Version = current_version();
//
//         match latest_version.cmp(&current_version) {
//             Ordering::Less => panic!("You are on a version newer than the latest version."),
//             Ordering::Equal => self.phase.lock().await.finish(),
//             Ordering::Greater => self.phase.lock().await.prompt(),
//         };
//
//         self.stop_doing_something()
//             .expect("Started doing something at the start of the function.");
//
//         // Ok.
//         Ok(())
//     }
// }
