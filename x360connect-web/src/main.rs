#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::http::CookieJar;
use rocket_db_pools::mongodb;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use crate::routes::games::download::get_file;
use crate::routes::games::upload::achievement_upload;
use crate::routes::games::upload::achievement_upload_i;
use crate::routes::games::upload::game_upload;
use crate::routes::keys::verify_key;
use crate::routes::login_req::login_req;
use crate::routes::profile::keys::create_profile_keys;
use crate::routes::profile::keys::delete_profile_keys;
use crate::routes::profile::keys::profile_keys;
use crate::routes::profile::profile::profile;
mod access_key;
mod routes;
mod user;
mod utils;
mod game;
mod document;
mod log_activity;
use routes::games;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

use routes::{login::login, auth::auth};


#[derive(Database)]
#[database("xboxrpc")]
struct MongoDB(mongodb::Client);

const DATABASE_NAME: &'static str = "xboxrpc";



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
    // let title_ids_file = str::from_utf8(include_bytes!(".././assets/titleids.json")).unwrap();
    // let titles: Vec<TitleInfo> = serde_json::from_str(title_ids_file).unwrap();

    // let mut title_ids: HashMap<String, activity::Activity> = HashMap::new();
    // for title in titles{
    //     let id = title.title_id.clone();
    //     let title = title.title;
    //     title_ids.insert(
    //         id.clone(), 
    //         activity::Activity { 
    //             title: title, 
    //             icon: format!("/assets/icons/{id}.png"),
    //             player: None
    //         }
    //     );
    // }

    rocket::build()
    .attach(Template::fairing())
    .attach(MongoDB::init())
    // .manage(title_ids)
    .mount("/assets", FileServer::from("./assets"))
    .mount("/", routes![
        index, games::game::game, auth, 
        login, login_req, 
        profile, profile_keys,
        create_profile_keys, delete_profile_keys,
        verify_key,
        game_upload,
        achievement_upload,
        achievement_upload_i,
        get_file
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
