/// SPDX-License-Identifier: GPL-3.0-only
use crate::{consts::ACHIEVEMENT_DISPLAY_TIME, structs::Achievement};
use color_eyre::eyre::{Result, bail};
use std::collections::VecDeque;
use tokio::time::Instant;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AchievementQueue {
    queue: VecDeque<Achievement>,
    displayed_at: Option<Instant>,
}

impl AchievementQueue {
    /// Returns how much time is left for the current achievement in the queue.
    ///
    /// # Errors
    /// This function will return an Err only if the queue is empty.
    pub fn seconds_left(&self) -> Result<f64> {
        if let Some(instant) = self.displayed_at {
            Ok(ACHIEVEMENT_DISPLAY_TIME - instant.elapsed().as_secs_f64())
        } else {
            let error_msg: &str =
                "Called the function AchievementQueue::seconds_left() when the queue is empty.";
            error!("{error_msg}");
            bail!("{error_msg}");
        }
    }

    pub fn queue_achievement<A>(&mut self, achievement: A)
    where
        A: Into<Achievement>,
    {
        if self.queue.is_empty() {
            self.displayed_at = Some(Instant::now());
        };
        self.queue.push_back(achievement.into());
    }

    pub fn finish_current(&mut self) {
        self.queue.pop_front();
        self.displayed_at = if self.queue.is_empty() {
            None
        } else {
            Some(Instant::now())
        };
    }

    pub fn current(&self) -> Option<&Achievement> {
        self.queue.front()
    }
}
