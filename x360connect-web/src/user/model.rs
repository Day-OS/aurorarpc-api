use std::vec;

use rocket::{http::CookieJar, time::Date};
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::{utils::generate_key, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "user";


#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct User { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub discord_id: String,
    pub profiles: Vec<Profile>,
    pub selected_profile: u8,
    pub username: String,
    pub nickname: String,
    pub access_keys: Vec<AccessKey>,
    pub screen_captures: Vec<ScreenCapture>
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct AccessKey{
    pub key: String,
    pub date: DateTime,
}


#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct Profile { 
    pub profile_name: String,
    pub gamerpoints: u32,
    pub profile_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct GameRecord { 
    pub game_id: String,
    pub achievements: Vec<u8>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenCapture{
    pub description: String,
    pub date: DateTime,
    pub resolution: (u32, u32)
}


impl User{
    pub async fn find_user_by_discord_id(db: &rocket_db_pools::mongodb::Client,id: String) -> anyhow::Result<Option<User>>{
        let collection = db.database(DATABASE_NAME)
            .collection::<User>(COLLECTION_NAME);
        Ok(collection.find_one(doc!{ "discord_id": id }, None).await?)
    }

    pub async fn new(
        db: &rocket_db_pools::mongodb::Client,
        id: String,
        username: String,
        
    ) -> anyhow::Result<User>{

        let user: User = User{
                id: None,
                discord_id: id,
                profiles: vec![],
                selected_profile: 0,
                username: username.clone(),
                nickname: username,
                screen_captures: vec![],
                access_keys: vec![],
            };

        db.database(DATABASE_NAME)
            .collection(COLLECTION_NAME)
            .insert_one(user.clone(), None)
            .await?;

        Ok(user)
    }

    pub async fn find_user_by_key(db: &rocket_db_pools::mongodb::Client, key: String) -> anyhow::Result<Option<User>>{
        let collection = db.database(DATABASE_NAME)
            .collection::<User>(COLLECTION_NAME);
        Ok(collection.find_one(doc!{ "access_keys.key": key }, None).await?)
    }

     pub async fn save(
        &self, 
        db: &rocket_db_pools::mongodb::Client,
    ) -> anyhow::Result<()>{
        let filter = doc! { "_id": &self.id };
        db.database(DATABASE_NAME)
            .collection::<Self>(COLLECTION_NAME)
            .replace_one(filter, self, None)
            .await?;

        Ok(())
    }

    pub async fn get_from_cookie(db: &rocket_db_pools::mongodb::Client, cookies: &CookieJar<'_>,) -> anyhow::Result<Option<Self>> {
        println!("asda");
        let discord_id = match cookies.get_private("discord_user_id") {
            Some(cookie) => {
            println!("sdasd");

                cookie.value().to_owned()
            },
            None => {
        println!("asdasddsfa");

                "WIP?".to_owned()
            },
        };
        Self::find_user_by_discord_id(db, discord_id).await
    }

    pub fn add_access_key(&mut self) -> &mut Self{
        let access_key = AccessKey { key: generate_key(), date: DateTime::now() };
        self.access_keys.push(access_key);
        self
    }

    pub fn remove_access_key(&mut self, index: usize) -> &mut Self{
        self.access_keys.remove(index);
        self
    }



}