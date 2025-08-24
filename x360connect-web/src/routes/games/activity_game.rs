use rocket::{http::Status, response::content::RawJson};
use rocket_db_pools::Connection;
use x360connect_global::{activity::{Activity, Player}, DEFAULT_AVATAR_IMAGE};

use crate::{access_key::OptionalAccessKeyGuard, modules::game::model::Game, MongoDB};

#[get("/activity/game/<id>")]
pub async fn activity_game(
    id: i64,
    access_key: OptionalAccessKeyGuard,
    db: Connection<MongoDB>
) -> Result<RawJson<String>, Status> {
    let mut player = None;
    if let Some(user) = access_key.0{
        let mut picture = DEFAULT_AVATAR_IMAGE.to_owned();

        if let Some(xuid) = user.selected_profile{
            let profile = user.profiles.get(&xuid);
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
    
    let mut game = Game::find_by_id(&db, id).await.map_err(|e| {
        println!("{e}");
        Status::InternalServerError
    })?.ok_or(
        Status::NotFound
    )?;

    if !game.images_were_downloaded{
        game.upload_own_images(&db).await.map_err(|e|{
            log::error!("Failed to upload own images of a game - {e}");
            Status::InternalServerError
        })?;
        game.achievements_were_downloaded = true;
        game.save(&db).await.map_err(|e|{
            log::error!("Failed to save a game - {e}");
            Status::InternalServerError
        })?;
    }

    let activity = Activity{ title: game.get_name(), icon: game.get_icon_url(), player: player};
    let json = serde_json::to_string(&activity).map_err(|e| {
        log::error!("{e}");
        Status::InternalServerError
    })?;
    Ok(
        RawJson(json)
    )
}