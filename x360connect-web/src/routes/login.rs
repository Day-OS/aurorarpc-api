use anyhow::Ok;
use rocket_dyn_templates::{context, Template};
use oauth2::{
    CsrfToken, Scope
};


#[get("/login")]
pub async fn login() -> Template {

    Template::render(
        "login", context! {
            site_name: "Rocket - Home Page",
            version: 127,
        }
    )

}