use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

use crate::{user::model::User, MongoDB};


#[get("/profile/keys")]
pub async fn profile_keys(
    cookies: &CookieJar<'_>, 
    db: Connection<MongoDB>
) -> Result<Template, Template> {
    let user = User::get_from_cookie(&db, cookies).await.unwrap();

    if let Some(user) = user {
        let access_tokens = user.access_keys;
        Ok(Template::render(
            "profile-keys", context! {
                access_tokens: access_tokens,
            }
        ))
    }
    else{
        Err(Template::render("error", context! {}))
    }
}


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