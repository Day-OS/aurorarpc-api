use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{modules::user::model::User, MongoDB};


#[get("/profile")]
pub async fn profile(
    cookies: &CookieJar<'_>, 
    db: Connection<MongoDB>
) -> Template {
    let user = User::get_from_cookie(&db, cookies).await.unwrap();
    match user{
        Some(user) => {
            Template::render(
                "profile", context! {
                    user: user
                }
            )
        },
        None => {
            Template::render(
                "login", context! {}
            )
        },
    }
    
}