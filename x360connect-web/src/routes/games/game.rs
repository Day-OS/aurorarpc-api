use rocket::{http::Status, response::content::RawJson};
use rocket_db_pools::Connection;
use x360connect_global::activity::{Activity, Player};

use crate::{access_key::OptionalAccessKeyGuard, game::model::Game, user::model::User, MongoDB};

#[get("/game/<id>")]
pub async fn game(
    id: &str,
    access_key: OptionalAccessKeyGuard,
    db: Connection<MongoDB>
) -> Result<RawJson<String>, Status> {
    let mut player = None;
    
    if let Some(key) = access_key.0{
        let user = User::find_user_by_key(&db, key).await.map_err(|e|{
            log::error!("{e}");
            Status::InternalServerError

        })?.ok_or(Status::Forbidden)?;
        let mut picture = "assets/default_image.png".to_owned();

        if let Some(index) = user.selected_profile{
            let profile = user.profiles.get_index(index as usize);
            if let Some(profile) = profile{
                picture = profile.avatar_url.clone();
            }
        }
        
        player = Some(
            Player{
                name: user.nickname,
                picture: picture, 
                url: format!("/u/{}", user.username) 
            }
        );
        
    }
    
    let game = Game::find_by_id(&db, id.to_string()).await.map_err(|e| {
        println!("{e}");
        Status::InternalServerError
    })?.ok_or(
        Status::NotFound
    )?;

    let activity = Activity{ title: game.get_name(), icon: game.get_icon_url(), player: player};
    let json = serde_json::to_string(&activity).map_err(|e| {
        log::error!("{e}");
        Status::InternalServerError
    })?;
    Ok(
        RawJson(json)
    )
}