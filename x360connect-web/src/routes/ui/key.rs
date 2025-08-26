use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{modules::user::model::User, MongoDB};

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