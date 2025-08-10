use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TitleInfo{
    #[serde(rename = "TitleID")]
    pub title_id: String,
    #[serde(rename = "Title")]
    pub title: String
}