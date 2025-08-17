use rocket_dyn_templates::{context, Template};


#[get("/login")]
pub async fn login() -> Template {

    Template::render(
        "login", context! {
            site_name: "Rocket - Home Page",
            version: 127,
        }
    )

}