use std::{collections::{HashMap, HashSet}, vec};

use ordermap::OrderSet;
use rocket::http::CookieJar;
use rocket_db_pools::mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::{document::{save, Document}, user::permission::Permission, utils::generate_key, DATABASE_NAME};

pub(crate) const COLLECTION_NAME: &'static str = "user";




#[derive(Debug, Serialize, Deserialize, Clone)] 
#[serde(crate = "rocket::serde")] 
pub struct User { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub discord_id: String,
    pub profiles: HashMap<String, Profile>,
    pub selected_profile: Option<String>,
    pub username: String,
    pub nickname: String,
    pub access_keys: OrderSet<AccessKey>,
    pub screen_captures: Vec<ScreenCapture>,
    pub permissions: HashSet<Permission>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] 
pub struct AccessKey{
    pub key: String,
    pub date: DateTime,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)] 
pub struct Profile { 
    pub avatar_url: String,
    pub gamertag: String,
    pub gamerscore: u32,
    pub needs_picture_update: bool, // This means the user has marked the profile to be reuploaded
    pub game_record: HashMap<String, Vec<u8>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenCapture{
    pub description: String,
    pub date: DateTime,
    pub resolution: (u32, u32)
}

impl Document for User{
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


impl User{
    pub async fn save(&self, db: &rocket_db_pools::mongodb::Client) -> anyhow::Result<()>{
        save(self, db).await?;
        Ok(())
    }

    pub async fn find_by_discord_id(db: &rocket_db_pools::mongodb::Client,id: String) -> anyhow::Result<Option<User>>{
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
                profiles: HashMap::new(),
                selected_profile: None,
                username: username.clone(),
                nickname: username,
                screen_captures: vec![],
                access_keys: OrderSet::new(),
                permissions: HashSet::new()
            };

        db.database(DATABASE_NAME)
            .collection(COLLECTION_NAME)
            .insert_one(user.clone(), None)
            .await?;

        Ok(user)
    }

    pub fn profile_image_name(&self, xuid: String) -> String{
        format!("profile_{}_{}", self.discord_id, xuid)
    }

    pub async fn find_user_by_key(db: &rocket_db_pools::mongodb::Client, key: String) -> anyhow::Result<Option<User>>{
        let collection = db.database(DATABASE_NAME)
            .collection::<User>(COLLECTION_NAME);
        Ok(collection.find_one(doc!{ "access_keys.key": key }, None).await?)
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
        Self::find_by_discord_id(db, discord_id).await
    }

    pub fn add_access_key(&mut self) -> &mut Self{
        let access_key = AccessKey { key: generate_key(), date: DateTime::now() };
        self.access_keys.insert(access_key);
        self
    }

    pub fn remove_access_key(&mut self, index: usize) -> &mut Self{
        self.access_keys.remove_index(index);
        self
    }

    pub fn check_permission(&self, permission: Permission) -> bool{
        let have_permission = self.permissions.get(&permission).is_some();
        if have_permission{
            return true
        }
        let have_all_permissions = self.permissions.get(&Permission::ALL).is_some();
        have_all_permissions
    }
}