use serde::{Deserialize, Serialize};
use crate::{modules::document::{save, Document}, DATABASE_NAME};

use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};

pub(crate) const COLLECTION_NAME: &'static str = "status";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct DatabaseStatus { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub games_filled: bool,
}

impl DatabaseStatus{
    pub async fn new(
        db: &rocket_db_pools::mongodb::Client,
        
    ) -> anyhow::Result<Self>{

        let document : Self = DatabaseStatus { id: None, games_filled: false };

        let collection = db.database(DATABASE_NAME)
            .collection::<DatabaseStatus>(COLLECTION_NAME);

        let result = collection.insert_one(document.clone(), None)
            .await?;


        let document = collection.find_one(doc!{ "_id": result.inserted_id }, None).await?.unwrap();

        Ok(document)
    }
    pub async fn save(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        save(self, db).await?;
        Ok(())
    }
    pub async fn get(db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<Self>{
        let collection = db.database(DATABASE_NAME)
            .collection::<DatabaseStatus>(COLLECTION_NAME);
        let mut status = collection
            .find_one(None, None)
            .await?;
        if status.is_none(){
            status = Some(DatabaseStatus::new(&db).await?);
        }
        
        Ok(status.unwrap())
    }
}


impl Document for DatabaseStatus{
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
