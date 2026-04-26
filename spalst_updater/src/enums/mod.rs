//! SPDX-License-Identifier: GPL-3.0-only

mod outcome;
// mod phase; // todo: maybe remove this (and the lines below!)
mod prompt_input;

pub(crate) use outcome::Outcome;
pub use outcome::Outcome as ProgramUpdateOutcome;
// pub(crate) use phase::Phase;
// pub use phase::Phase as ProgramUpdatePhase;
pub use prompt_input::PromptInput as ProgramUpdatePromptInput;
pub(crate) use prompt_input::PromptInput;
