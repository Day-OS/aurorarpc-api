use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{modules::user::model::User, MongoDB};

#[get("/")]
pub async fn index(
    cookies: &CookieJar<'_>, 
    db: Connection<MongoDB>
) -> Result<Template, Template> {
    Ok(Template::render("index", context! {}))
}