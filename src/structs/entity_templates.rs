/// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    enums::MergePriority,
    structs::{Entity, EntityTemplateName},
    traits::Loadable,
};
use color_eyre::eyre::{Context, Result, bail};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Default, Deserialize)]
pub struct EntityTemplates(HashMap<EntityTemplateName, Entity>);

impl Loadable for EntityTemplates {}

impl EntityTemplates {
    pub fn spawn(&self, template: EntityTemplateName) -> Option<Entity> {
        self.0.get(&template).cloned()
    }

    /// Processes the given directory and returns the `EntityTemplates` instance processed.
    /// The argument `path_to_process` should always be None.
    /// The argument `duplicates_possible` should be `true` if duplicates are possible, and if the
    /// program encounters a duplilcate, it will return an Err. If it is set to `false` and the
    /// program encounters a duplicate, `unreachable!()` will be called.
    pub fn process_dir(
        original_path: &Path,
        path_to_process: Option<&Path>,
        duplicates_possible: bool,
    ) -> Result<Self> {
        let path: &Path = if let Some(p) = path_to_process {
            p
        } else {
            original_path
        };
        let mut entity_templates: EntityTemplates = EntityTemplates::default();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if path == original_path
                && path.file_name() == Some("entity_templates".as_ref())
                && entry.file_name() == "core"
                && entry_path.is_dir()
            {
                continue;
            };

            if entry_path.is_dir() {
                EntityTemplates::process_dir(
                    original_path,
                    Some(&entry_path),
                    duplicates_possible,
                )?;
            } else if entry_path.is_file() {
                if entry_path.extension() != Some("ron".as_ref()) {
                    bail!(
                        "Found non-.ron file inside the entity_templates directory ({}) at location {} inside the directory {}.",
                        original_path.display(),
                        entry_path.display(),
                        path.display(),
                    )
                };
                // Now, the path extension is for sure "ron", so it's safe to merge it in
                let merge_priority: MergePriority = if duplicates_possible {
                    MergePriority::Error
                } else {
                    MergePriority::Unreachable
                };
                entity_templates.merge(
                    EntityTemplates::load(&entry_path).wrap_err_with(|| {
                        format!(
                            "Tried loading entity templates from path {}.",
                            entry_path.display()
                        )
                    })?,
                    merge_priority,
                )?;
            } else {
                bail!(
                    "Found an element inside the entry_templates directory ({}) at location {} inside the directory {} that is not a file.",
                    original_path.display(),
                    entry_path.display(),
                    path.display()
                );
            }
        }
        // Ok.
        Ok(entity_templates)
    }

    pub fn merge(&mut self, other: Self, priority: MergePriority) -> Result<()> {
        match priority {
            MergePriority::Self_ => {
                for (key, value) in other.0 {
                    self.0.entry(key).or_insert(value);
                }
            }
            MergePriority::Other => {
                for (key, value) in other.0 {
                    self.0.insert(key, value);
                }
            }
            MergePriority::Error | MergePriority::Unreachable => {
                for (key, value) in other.0 {
                    if self.0.contains_key(&key) {
                        if priority == MergePriority::Unreachable {
                            unreachable!();
                        } else {
                            bail!(
                                "While merging EntityTemplates with the MergePriority set to Error, encountered double keys \"{key}\".",
                            );
                        };
                    };
                    self.0.insert(key, value);
                }
            }
        };
        // Ok.
        Ok(())
    }
}
