use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use rocket::http::Status;

use crate::{modules::user::model::User, MongoDB};

#[post("/profile/keys")]
pub async fn create_profile_keys(
    cookies: &CookieJar<'_>, 
    db: Connection<MongoDB>
) -> Status {
    let user = User::get_from_cookie(&db, cookies).await.unwrap();

    if let Some(mut user) = user {
        user.add_access_key().save(&db).await.unwrap();
        Status::Ok
    }
    else{
        Status::Forbidden
    }
}

#[delete("/profile/keys?<key>")]
pub async fn delete_profile_keys(
    key: u8,
    cookies: &CookieJar<'_>, 
    db: Connection<MongoDB>
) -> Status {
    let user = User::get_from_cookie(&db, cookies).await.unwrap();

    if let Some(mut user) = user {
        user.remove_access_key(key.into()).save(&db).await.unwrap();
        Status::Ok
    }
    else{
        Status::Forbidden
    }
}