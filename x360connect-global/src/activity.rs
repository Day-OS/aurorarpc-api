use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Activity{
    pub title: String,
    pub icon: String,
    pub player: Option<Player>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player{
    pub name: String,
    pub picture: String,
    pub url: String,
}
