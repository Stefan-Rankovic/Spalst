//! SPDX-License-Identifier: GPL-3.0-only

/// How much the `MenuThing` is pinned.
///
/// If two `MenuThing`s have the same `MenuThingPinnedStatus` and also have the same `priority`,
/// there are no guarantees on which will be rendered first. I don't even know yet, didn't code
/// that. todo.
///
/// Note that you don't have to pin `MenuThing`s to the top of the screen/whatever, you can pin it down!
/// Not sure how that works yet though. So this is subject to change. todo!
#[derive(Clone, Copy, Debug, Default)]
pub enum MenuThingPinnedStatus {
    /// Pinned to the bottom. Not sure where this could be used.
    BottomOfScreen {
        /// Higher priority = more down.
        priority: u8,
    },
    /// Pinned below the parent of the `MenuThing`. Not sure where this could be used.
    BottomOfParent {
        /// Higher priority = more down. Regardless of its value, it will always be above earlier
        /// discriminants.
        priority: u8,
    },
    /// Wherever it can fit.
    #[default]
    NotPinned,
    /// Pinned above the parent of the `MenuThing`.
    TopOfParent {
        /// Higher priority = higher up. Regardless of its value, it will always be above earlier
        /// discriminants.
        priority: u8,
    },
    /// Pinned at the top.
    TopOfScreen {
        /// Higher priority = higher up. Regardless of its value, it will always be above earlier
        /// discriminants.
        priority: u8,
    },
}
