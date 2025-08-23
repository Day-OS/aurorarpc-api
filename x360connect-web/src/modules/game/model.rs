use anyhow::{anyhow, Ok};
use ordermap::OrderSet;
use reqwest::StatusCode;
use rocket::http::Status;
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use url::Url;
use x360connect_global::schm_game::Images;
use x360connect_global::DEFAULT_BIG_IMAGE;
use x360connect_global::{schm_achivements, schm_game::SchmGame};
use crate::rocket::futures::AsyncWriteExt;

use crate::{modules::document::{save, Document}, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "game";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct Game { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub game_id: i64,
    pub schema: SchmGame,
    pub images_were_downloaded: bool,
    pub achievements_were_downloaded: bool,
    pub achievements: OrderSet<Achievement>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] 
pub struct Achievement{
    pub id: u32,
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
        crate::modules::document::new(self, db).await?;
        Ok(())
    }
    pub async fn save(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        save(self, db).await?;
        Ok(())
    }


    pub async fn find_by_id(db: &rocket_db_pools::mongodb::Client,id: i64) -> anyhow::Result<Option<Self>>{
        let collection = db.database(DATABASE_NAME)
            .collection::<Self>(COLLECTION_NAME);
        Ok(collection.find_one(doc!{ "game_id": id }, None).await?)
    }

    pub fn achivement_image_name(&self, uuid: String) -> String{
        format!("{}_achievement_{}", self.game_id, uuid)
    }

    pub fn get_name(&self) -> String{
        self.schema.fulltitle.clone()
    }
    pub fn get_icon_url(&self) -> String{
        if let Some(images) = &self.schema.images{
            if let Some(icon) = &images.icon{
                return icon.clone()
            }
        }
        return DEFAULT_BIG_IMAGE.to_owned()
    }

    pub async fn upload_own_images(&mut self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        if self.schema.images.is_none(){
            return Ok(())
        }
    
        let new_images = if let Some(images) = self.schema.images.clone() {
            Images {
                boxart: self._upload_image(images.boxart.clone(), "boxart", db).await?,
                icon: self._upload_image(images.icon.clone(), "icon", db).await?,
                banner: self._upload_image(images.banner.clone(), "banner", db).await?,
            }
        } else {
            Images {
                boxart: None,
                icon: None,
                banner: None,
            }
        };
        self.schema.images = Some(new_images);
        Ok(())
    }


    async fn _upload_image(&self, original_url: Option<String>, category: &str, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<Option<String>>{
        log::debug!("Uploading {} image", category);
        if original_url.is_none(){
            return Ok(None);
        }
        let original_url = original_url.unwrap();
        let new_name = format!("{}_{}", self.game_id, category);
        upload_image(db, original_url, new_name.clone()).await?;
        Ok(Some(new_name))
    }

}


async fn upload_image(db: &rocket_db_pools::mongodb::Client, original_url: String, file_name: String) -> anyhow::Result<()>{
    let client = reqwest::Client::new();
    
    // If the URL of the image is not an actual URL, then we just ignore it
    if Url::parse(&original_url).is_err(){
        return Ok(())
    }

    let response = client.get(original_url.clone()).send().await.map_err(|e|{
        log::error!("Error while trying to send request to download image from Aurora | URL:{original_url}");
        e
    })?;
    let status = response.status();

    if status != StatusCode::OK{
        log::error!("Error while trying to gather image from Aurora | URL:{original_url} - status: {status:#?}");
        return Err(anyhow!(status));
    }
    
    let buf = response.bytes().await?;

    let bucket = db.database(DATABASE_NAME).gridfs_bucket(None);
    let mut upload_stream = bucket.open_upload_stream(file_name.clone(), None);
    upload_stream.write_all(&buf).await.map_err(|e|{
        log::error!("Error while trying to write into upload stream {e}");
        anyhow!(Status::InternalServerError)
    })?;
    upload_stream.close().await.map_err(|e|{
        log::error!("Error while trying to close upload_stream {e}");
        anyhow!(Status::InternalServerError)
    })?;

    Ok(())
}