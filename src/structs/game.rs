//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    enums::GameMenu,
    structs::{Entity, EntityId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Game {
    menu: GameMenu,
    entities: HashMap<EntityId, Entity>,
    next_entity_id: EntityId,
    player_id: EntityId,
}

impl Game {
    pub fn add_entity(&mut self, entity: Entity) {
        self.next_entity_id.0 += 1;
        if self.entities.insert(self.next_entity_id, entity).is_some() {
            unreachable!();
        };
    }
}
