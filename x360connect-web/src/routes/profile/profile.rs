use anyhow::Ok;
use rocket_dyn_templates::{context, Template};
use oauth2::{
    CsrfToken, Scope
};


#[get("/profile")]
pub async fn profile() -> Template {
    Template::render(
        "profile", context! {
            site_name: "Rocket - Home Page",
            version: 127,
        }
    )
}