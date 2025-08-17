use ordermap::OrderSet;
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use x360connect_global::{schm_achivements, schm_game::SchmGame};

use crate::{document::{save, Document}, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "game";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct Game { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub game_id: String,
    pub schema: SchmGame,
    pub achievements: OrderSet<Achievement>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] 
pub struct Achievement{
    pub id: String,
    pub schema: schm_achivements::Achievement,
    pub icon_url: String,
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
    pub async fn new(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        crate::document::new(self, db).await?;
        Ok(())
    }
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

    pub fn get_name(&self) -> String{
        self.schema.fulltitle.clone().unwrap_or(
            self.schema.title_id.clone()
            .unwrap_or("Undefined".to_owned()
        ))
    }
    pub fn get_icon_url(&self) -> String{
        if let Some(images) = &self.schema.images{
            if let Some(icon) = &images.icon{
                return icon.clone()
            }
        }
        return "xbox-360-logo".to_owned()
    }
}