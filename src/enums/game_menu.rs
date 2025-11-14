use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum GameMenu {
    #[default]
    Todo,
}
