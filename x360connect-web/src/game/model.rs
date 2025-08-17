use anyhow::{anyhow, Ok};
use ordermap::OrderSet;
use reqwest::StatusCode;
use rocket::http::Status;
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use x360connect_global::schm_game::{BoxArt, Images};
use x360connect_global::{schm_achivements, schm_game::SchmGame};
use crate::rocket::futures::AsyncWriteExt;

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

    pub async fn upload_own_images(&mut self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        if self.schema.images.is_none(){
            return Ok(())
        }
    
        let new_images = if let Some(images) = self.schema.images.clone() {
            let mut new_screenshots = vec![];
            if let Some(screenshots) = images.screenshots.clone() {
                for (i, image) in screenshots.iter().enumerate() {
                    let image_name = format!("screenshot_{}", i);
                    let uploaded = self._upload_image(Some(image.clone()), &image_name, db).await?;
                    if let Some(url) = uploaded {
                        new_screenshots.push(url);
                    }
                }
            }
            let mut box_art = None;
            if let Some(boxart) = images.boxart{
                box_art = Some(BoxArt{
                    small: self._upload_image(boxart.small.clone(), "boxart_small", db).await?,
                    large: self._upload_image(boxart.large.clone(), "boxart_large", db).await?,
                })
            }

            Images {
                screenshots: Some(new_screenshots),
                boxart: box_art,
                icon: self._upload_image(images.icon.clone(), "icon", db).await?,
                background: self._upload_image(images.background.clone(), "background", db).await?,
                banner: self._upload_image(images.banner.clone(), "banner", db).await?,
            }
        } else {
            Images {
                screenshots: None,
                boxart: None,
                icon: None,
                background: None,
                banner: None,
            }
        };
        self.schema.images = Some(new_images);
        Ok(())
    }


    async fn _upload_image(&self, original_url: Option<String>, category: &str, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<Option<String>>{
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
    let response = client.get(original_url).send().await?;
    let status = response.status();

    if status != StatusCode::OK{
        return Err(anyhow!(status))
    }
    
    let buf = response.bytes().await?;

    let bucket = db.database(DATABASE_NAME).gridfs_bucket(None);
    let mut upload_stream = bucket.open_upload_stream(file_name.clone(), None);
    upload_stream.write_all(&buf).await.map_err(|e|{
        log::error!("{e}");
        anyhow!(Status::InternalServerError)
    })?;
    upload_stream.close().await.map_err(|e|{
        log::error!("{e}");
        anyhow!(Status::InternalServerError)
    })?;

    Ok(())
}