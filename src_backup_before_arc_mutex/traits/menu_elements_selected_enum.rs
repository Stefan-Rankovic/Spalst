//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{DOWN_KEYS, LEFT_KEYS, RIGHT_KEYS, UP_KEYS},
    traits::{EnumAsStr, MenuElement},
};
use color_eyre::eyre::{Result, bail};
use crossterm::event::KeyCode;
use std::fmt::Debug;
use strum::{EnumCount, IntoDiscriminant, IntoEnumIterator};

/// Defines an enum menu with variants as menu elements.
///
/// The enum variants should only hold data related to selection. Everything else should be handled
/// by the corresponding MenuElements struct.
pub trait MenuElementsSelectedEnum:
    Copy + Debug + EnumAsStr + EnumCount + IntoDiscriminant
where
    Self::Discriminant: IntoEnumIterator,
{
    /// Select the element left of the current one.
    fn select_left(&self) -> Option<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Sized;
    /// Select the element below the current one.
    fn select_down(&self) -> Option<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Sized;
    /// Select the element above the current one.
    fn select_up(&self) -> Option<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Sized;
    /// Select the element right of the current one.
    fn select_right(&self) -> Option<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Sized;

    /// Calls self.select_left(), and if the element is out of bounds, return copied self.
    fn select_left_no_out_of_bounds(&self) -> <Self as IntoDiscriminant>::Discriminant
    where
        Self: Sized,
    {
        self.select_left().unwrap_or_else(|| self.discriminant())
    }
    /// Calls self.select_down(), and if the element is out of bounds, return copied self.
    fn select_down_no_out_of_bounds(&self) -> <Self as IntoDiscriminant>::Discriminant
    where
        Self: Sized,
    {
        self.select_down().unwrap_or_else(|| self.discriminant())
    }
    /// Calls self.select_up(), and if the element is out of bounds, return copied self.
    fn select_up_no_out_of_bounds(&self) -> <Self as IntoDiscriminant>::Discriminant
    where
        Self: Sized,
    {
        self.select_up().unwrap_or_else(|| self.discriminant())
    }
    /// Calls self.select_right(), and if the element is out of bounds, return copied self.
    fn select_right_no_out_of_bounds(&self) -> <Self as IntoDiscriminant>::Discriminant
    where
        Self: Sized,
    {
        self.select_right().unwrap_or_else(|| self.discriminant())
    }

    /// Calls self.select_decide(), and if it fails, ignores the error and returns copied self.
    fn select_decide_safe(&self, key: &KeyCode) -> Option<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Copy + Sized,
    {
        self.select_decide(key)
            .unwrap_or_else(|_| Some(self.discriminant()))
    }
    /// Based on the passed key, call one of the self.select_...() methods.
    fn select_decide(
        &self,
        key: &KeyCode,
    ) -> Result<Option<<Self as IntoDiscriminant>::Discriminant>>
    where
        Self: Sized,
    {
        Ok(if LEFT_KEYS.contains(key) {
            self.select_left()
        } else if DOWN_KEYS.contains(key) {
            self.select_down()
        } else if UP_KEYS.contains(key) {
            self.select_up()
        } else if RIGHT_KEYS.contains(key) {
            self.select_right()
        } else {
            bail!("Passed key {key} is not a movement key.")
        })
    }

    /// Calls self.select_decide_safe(), and if it fails or the element is out of bounds, returns
    /// copied self.
    fn select_decide_safe_no_out_of_bounds(
        &self,
        key: &KeyCode,
    ) -> <Self as IntoDiscriminant>::Discriminant
    where
        Self: Sized,
    {
        self.select_decide_safe(key)
            .unwrap_or_else(|| self.discriminant())
    }

    /// Calls self.select_decide() and if the element is out of bounds, returns copied self.
    ///
    /// If self.select_decide() returns an error, the error will not be ignored.
    fn select_decide_no_out_of_bounds(
        &self,
        key: &KeyCode,
    ) -> Result<<Self as IntoDiscriminant>::Discriminant>
    where
        Self: Sized,
    {
        self.select_decide(key)
            .map(|option: Option<<Self as IntoDiscriminant>::Discriminant>| ->
                <Self as IntoDiscriminant>::Discriminant {option.unwrap_or_else(|| self.discriminant())})
    }
}
