//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{DOWN_KEYS, ESCAPE_KEYS, LEFT_KEYS, RIGHT_KEYS, UP_KEYS},
    enums::{
        MainMenuEnum, ManagePlaythroughsMenu, ManagePlaythroughsSelected as Selected,
        PlaythroughsSortBy as SortBy, Select,
    },
    structs::{App, Playthrough, PlaythroughName},
};
use color_eyre::eyre::{OptionExt, Result};
use ratatui::crossterm::event::{Event, KeyCode};
use strum::IntoEnumIterator;

impl App {
    pub fn manage_playthroughs_select_handle_event(&mut self, event: Event) -> Result<()> {
        let MainMenuEnum::ManagePlaythroughs(ManagePlaythroughsMenu::Select {
            selected,
            sort_ascending,
            sort_by,
        }) = self.menu().current()
        else {
            unreachable!()
        };
        if let Event::Key(key) = event {
            if selected.is_playthroughs() {
                let Selected::Playthroughs {
                    selected: selected_name,
                } = selected
                else {
                    unreachable!()
                };
                match key.code {
                    code if DOWN_KEYS.contains(&code) || UP_KEYS.contains(&code) => {
                        let new_selected: PlaythroughName =
                            self.account.playthroughs.get_new_playthrough(
                                selected_name,
                                if DOWN_KEYS.contains(&code) {
                                    Select::Next
                                } else {
                                    Select::Previous
                                },
                                *sort_by,
                                *sort_ascending,
                            )?;
                        let sort_ascending: bool = *sort_ascending;
                        let sort_by: SortBy = *sort_by;
                        self.menu_mut().set_same(MainMenuEnum::ManagePlaythroughs(
                            ManagePlaythroughsMenu::Select {
                                selected: Selected::Playthroughs {
                                    selected: new_selected,
                                },
                                sort_ascending,
                                sort_by,
                            },
                        ))?;
                    }
                    code if ESCAPE_KEYS.contains(&code) => {
                        let sort_ascending: bool = *sort_ascending;
                        let sort_by: SortBy = *sort_by;
                        self.menu_mut().set_same(MainMenuEnum::ManagePlaythroughs(
                            ManagePlaythroughsMenu::Select {
                                selected: Selected::SortBy,
                                sort_ascending,
                                sort_by,
                            },
                        ))?
                    }
                    code if RIGHT_KEYS.contains(&code) || code == KeyCode::Enter => {
                        let selected_name: PlaythroughName = selected_name.clone();
                        self.menu_mut().set_same(MainMenuEnum::ManagePlaythroughs(
                            ManagePlaythroughsMenu::Playthrough(selected_name),
                        ))?
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    code if LEFT_KEYS.contains(&code)
                        || DOWN_KEYS.contains(&code)
                        || UP_KEYS.contains(&code)
                        || RIGHT_KEYS.contains(&code) =>
                    {
                        let default_playthrough: PlaythroughName =
                            self.account.playthroughs.sorted(*sort_by, *sort_ascending)[0]
                                .0
                                .clone();
                        let selected = selected.select_decide(&code, default_playthrough)?;
                        let sort_ascending: bool = *sort_ascending;
                        let sort_by: SortBy = *sort_by;
                        self.menu_mut().set_same(MainMenuEnum::ManagePlaythroughs(
                            ManagePlaythroughsMenu::Select {
                                selected,
                                sort_ascending,
                                sort_by,
                            },
                        ))?;
                    }
                    KeyCode::Enter => {
                        let selected: Selected = selected.clone();
                        let sort_ascending: bool = *sort_ascending;
                        let sort_by: SortBy = *sort_by;
                        self.menu_mut().set_same(MainMenuEnum::ManagePlaythroughs(
                            ManagePlaythroughsMenu::Select {
                                sort_ascending: if selected == Selected::SortAscending {
                                    !sort_ascending
                                } else {
                                    sort_ascending
                                },
                                sort_by: if selected == Selected::SortBy {
                                    SortBy::iter()
                                        .skip_while(|sb: &SortBy| -> bool { *sb != sort_by })
                                        .nth(1)
                                        .unwrap_or_else(|| SortBy::iter().next().unwrap())
                                } else {
                                    sort_by
                                },
                                selected: selected.clone(),
                            },
                        ))?;
                    }
                    code if ESCAPE_KEYS.contains(&code) => self.menu_mut().browse(),
                    _ => {}
                };
            };
        };
        // Ok.
        Ok(())
    }
}
