use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::{modules::document::{new, Document}, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "log";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct Log { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub discord_id: String,
    pub log_type: LogType
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(tag = "type")]
pub enum LogType{
    UploadGameInfo{
        game_id: i64,
    },
    UploadGameAchievementInfo{
        game_id: i64,
        id: String,
    },
    UploadGameAchievementImage{
        game_id: i64,
        id: String,
    }
}

impl Document for Log{
    fn database_name(&self) -> String {
        DATABASE_NAME.to_owned()
    }

    fn collection_name(&self) -> String {
        COLLECTION_NAME.to_owned()
    }

    fn id(&self) -> rocket_db_pools::mongodb::bson::oid::ObjectId {
        self.id.expect("id should be present")
    }
}

impl Log{
    pub async fn new(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        new(self, db).await?;
        Ok(())
    }
}