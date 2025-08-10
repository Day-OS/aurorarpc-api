#[macro_use] extern crate rocket;
use std::collections::HashMap;

use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::http::CookieJar;
use rocket::response::content::RawJson;
use rocket::State;
use rocket_db_pools::mongodb;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use x360connect_global::activity;
pub mod access_key;

use crate::game_data::{TitleInfo};
use crate::routes::keys::verify_key;
use crate::routes::login_req::login_req;
use crate::routes::profile::keys::create_profile_keys;
use crate::routes::profile::keys::delete_profile_keys;
use crate::routes::profile::{profile::profile, keys::profile_keys};
mod game_data;
use serde_json;
mod routes;
mod user;
mod utils;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

use routes::{login::login, auth::auth};


#[derive(Database)]
#[database("xboxrpc")]
struct MongoDB(mongodb::Client);

const DATABASE_NAME: &'static str = "xboxrpc";



#[get("/game/<id>")]
fn game(id: String, info: &State<HashMap<String, activity::Activity>>) -> RawJson<String> {
    let game_data = match info.get(&id) {
        Some(game) => game,
        None => &activity::Activity{title: "UNDEFINED".to_string(), icon: "-1".to_string(), player: None},
    };
    RawJson(serde_json::to_string(&game_data).unwrap())
}



#[get("/")]
fn index(cookies: &CookieJar<'_>,) -> String {
    println!("asda");
    match cookies.get_private("discord_user_id") {
        Some(cookie) => {
        println!("sdasd");

            cookie.value().to_owned()
        },
        None => {
    println!("asdasddsfa");

            "WIP?".to_owned()
        },
    }

}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let title_ids_file = str::from_utf8(include_bytes!(".././assets/titleids.json")).unwrap();
    let titles: Vec<TitleInfo> = serde_json::from_str(title_ids_file).unwrap();

    let mut title_ids: HashMap<String, activity::Activity> = HashMap::new();
    for title in titles{
        let id = title.title_id.clone();
        let title = title.title;
        title_ids.insert(
            id.clone(), 
            activity::Activity { 
                title: title, 
                icon: format!("/assets/icons/{id}.png"),
                player: None
            }
        );
    }

    rocket::build()
    .attach(Template::fairing())
    .attach(MongoDB::init())
    .manage(title_ids)
    .mount("/assets", FileServer::from("./assets"))
    .mount("/", routes![
        index, game, auth, 
        login, login_req, 
        profile, profile_keys, 
        create_profile_keys, delete_profile_keys,
        verify_key
    ])
    .mount("/", openapi_get_routes![])
    .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
