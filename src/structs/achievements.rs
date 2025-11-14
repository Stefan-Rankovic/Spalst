use crate::{
    enums::AchievementId,
    structs::AchievementInfo,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct Achievements(HashMap<AchievementId, AchievementInfo>);

impl Default for Achievements {
    fn default() -> Self {
        let mut achievement_id_and_info: HashMap<AchievementId, AchievementInfo> = HashMap::new();
        for id in AchievementId::iter() {
            achievement_id_and_info.insert(id, AchievementInfo::from_id(id));
        };
        Self(achievement_id_and_info)
    }
}
