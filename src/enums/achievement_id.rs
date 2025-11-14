use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum AchievementId {
    Hello,
}
