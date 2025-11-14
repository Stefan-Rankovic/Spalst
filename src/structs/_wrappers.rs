use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityId(pub u64);

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlaythroughId(pub u64);

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SaveId(pub u64);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlaythroughName(pub String);
