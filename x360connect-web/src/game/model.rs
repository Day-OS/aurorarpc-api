use ordermap::OrderSet;
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::{document::{save, Document}, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "game";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct Game { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub game_id: String,
    pub icon_url: String,
    pub title: String,
    pub description: String,
    pub developer: String,
    pub publisher: String,
    pub images_url: Vec<String>,
    pub achievements: OrderSet<Achievement>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] 
pub enum AchievementType{

}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] 
pub struct Achievement{
    pub id: String,
    pub icon_url: String,
    pub title: String,
    pub description: String,
    pub gamescode: u8,
    pub live_dependant: bool,
    pub achvmnt_type: AchievementType
}

impl Document for Game{
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


impl Game{
    pub async fn save(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        save(self, db).await?;
        Ok(())
    }


    pub async fn find_by_id(db: &rocket_db_pools::mongodb::Client,id: String) -> anyhow::Result<Option<Self>>{
        let collection = db.database(DATABASE_NAME)
            .collection::<Self>(COLLECTION_NAME);
        Ok(collection.find_one(doc!{ "game_id": id }, None).await?)
    }

    pub fn achivement_image_name(&self, uuid: String) -> String{
        format!("{}_achievement_{}", self.game_id, uuid)
    }
}