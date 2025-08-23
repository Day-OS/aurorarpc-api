use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchmGame {
    pub fulltitle: String,
    pub title_id: i64,
    pub description: Option<String>,
    pub categories: Option<Vec<Category>>,
    pub reduced_title: Option<String>,
    pub release_date: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub rating_aggregate: Option<String>,
    pub images: Option<Images>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub categoryid: String,
    pub system: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub boxart: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
}