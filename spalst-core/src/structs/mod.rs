//! SPDX-License-Identifier: GPL-3.0-only

mod account;
mod achievement;
mod achievement_queue;
mod app;
mod args_parser;
mod asset;
mod data;
mod ensure_terminal_restore;
mod entity;
mod entity_id;
mod select_amount;
// mod entity_template_name; // todo: delete the file
// mod entity_templates; // todo: delete the file
mod game;
mod item;
mod menu;
mod menu_element;
mod menu_elements;
mod playthrough;
mod playthrough_id;
mod playthrough_name;
mod playthroughs;
mod raw_release;
mod release;
mod release_unsafety_reason;
mod releases;
mod save;
mod save_id;
mod sort_descending;
mod sorter;
mod stats;
mod unsafe_version;
// mod unsafe_versions; // todo: remove this line
// mod duration; // todo: remove this line

// pub use duration::Duration; // todo: remove this line
// pub use unsafe_versions::UnsafeVersions; // todo: remove this line
pub use account::Account;
pub use achievement::Achievement;
pub use achievement_queue::AchievementQueue;
pub use app::App;
pub use args_parser::ArgsParser;
pub use asset::Asset;
pub use data::Data;
pub use ensure_terminal_restore::EnsureTerminalRestore;
pub use entity::Entity;
pub use entity_id::EntityId;
pub use game::Game;
pub use item::Item;
pub use menu::Menu;
pub use menu_element::{MenuElementEmpty, MenuElementKeyValue, MenuElementList, MenuElementRaw};
pub use menu_elements::{
    MenuElementsMainMenu, MenuElementsManagePlaythrough, MenuElementsSortableList,
};
pub use playthrough::Playthrough;
pub use playthrough_id::PlaythroughId;
pub use playthrough_name::PlaythroughName;
pub use playthroughs::Playthroughs;
pub use raw_release::RawRelease;
pub use release::Release;
pub use release_unsafety_reason::ReleaseUnsafetyReason;
pub use releases::Releases;
pub use save::Save;
pub use save_id::SaveId;
pub use select_amount::SelectAmount;
pub use sort_descending::SortDescending;
pub use sorter::Sorter;
pub use stats::Stats;
pub use unsafe_version::UnsafeVersion;
