/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::ENTITY_TEMPLATES_PATH,
    enums::{AchievementId, MergePriority},
    structs::{Achievement, EntityTemplates},
    traits::Loadable,
};
use color_eyre::eyre::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Data {
    entity_templates: EntityTemplates,
}

impl Data {
    pub async fn try_new(game_path: &Path) -> Result<Self> {
        let entity_templates: EntityTemplates = {
            // The path of the directory entity_templates.
            let entity_templates_path: &Path = &game_path.join(ENTITY_TEMPLATES_PATH);
            // The entity templates inside .../entity_templates/core/.
            let mut entity_templates_core: EntityTemplates =
                EntityTemplates::process_dir(&entity_templates_path.join("core"), None, false)
                    .await?;
            // Merge the templates inside core with the templates outside core.
            entity_templates_core.merge(
                EntityTemplates::process_dir(entity_templates_path, None, true).await?,
                MergePriority::Other,
            )?;
            entity_templates_core
        };
        Ok(Self { entity_templates })
    }
}

impl Data {
    pub fn entity_templates(&self) -> &EntityTemplates {
        &self.entity_templates
    }
}
