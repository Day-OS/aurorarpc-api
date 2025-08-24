use rocket::{data::ToByteUnit, futures::AsyncWriteExt, http::Status, serde::json::Json, tokio::io::AsyncReadExt, Data};
use rocket_db_pools::Connection;
use x360connect_global::schm_achivements;

use crate::{access_key::AccessKeyGuard, log_activity::Log, modules::game::model::{Achievement, Game}, MongoDB, DATABASE_NAME};


#[get("/achievement_upload/<id>")]
pub async fn are_achievements_uploaded<'r>(
    id: i64,
    _access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
) -> Result<Status, Status> {
    let game = Game::find_by_id(&db, id)
        .await.map_err(|e|{
            error!("{e}");
            Status::InternalServerError
    })?;

    match game{
        Some(game) => {
            if game.achievements_were_downloaded{
                Ok(Status::Ok)
            }
            else{
                Ok(Status::NoContent)
            }
        },
        None => 
        return Err(Status::NotFound),
    }
}

#[post("/achievement_upload/<id>", data = "<input>")]
pub async fn achievement_upload<'r>(
    id: i64,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    input: Json<Vec<schm_achivements::Achievement>>
) -> Result<Status, Status> {
   
    let user = access_key.0;

    let game_already_exists = Game::find_by_id(&db, id)
        .await.map_err(|e|{
            error!("{e}");
            Status::InternalServerError
        })?;
    
    if game_already_exists.is_none(){
        return Err(Status::NotFound);
    }

    let mut game = game_already_exists.unwrap();

    if game.achievements_were_downloaded{
        return Err(Status::Conflict);
    }

    for achievement in input.0{
        let id = achievement.id.clone();
        let result = Achievement{ 
            schema: achievement, 
            icon_url: None
        };
        game.achievements.insert(id.to_string(), result);
    }

    game.achievements_were_downloaded = true;
    let game_copy = game.clone();
    _ = game.save(&db).await.map_err(|e|{
        log::error!("Could not save game. {e}");
        log::error!("Could not save game. {game_copy:?}");
    });

    let log = Log{ 
        id: None, 
        discord_id: user.discord_id, 
        log_type: crate::log_activity::LogType::UploadGameAchievementInfo { game_id: id }
    };
    _ = log.new(&db).await.map_err(|e|{
        log::error!("Could not save log. {e}");
    });
    

    Ok(
        Status::Ok
    )
}


#[post("/achievement_upload_i/<id>/<uuid>", data = "<image>")]
pub async fn achievement_upload_i<'r>(
    id: i64,
    uuid: u32,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    image: Data<'_>
) -> Result<Status, Status> {   
    let user = access_key.0;

    let mut buf = Vec::new();
    image.open(10.mebibytes()).read_to_end(&mut buf).await.unwrap();

    let mut game = Game::find_by_id(&db, id).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::NotFound)?;
    let file_name = game.achivement_image_name(uuid.to_owned());

    let achievements = game.achievements.get_mut(&uuid.to_string());

    if achievements.is_none(){
        return Err(Status::NotFound)
    }

    let achievement = achievements.unwrap();

    if achievement.icon_url.is_some(){
        return Err(Status::Conflict)
    }
    
    achievement.icon_url = Some(file_name.clone());

    game.save(&db).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;

    let bucket = db.database(DATABASE_NAME).gridfs_bucket(None);
    let mut upload_stream = bucket.open_upload_stream(file_name, None);
    upload_stream.write_all(&buf).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;
    upload_stream.close().await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;

     let log = Log{ 
        id: None, 
        discord_id: user.discord_id, 
        log_type: crate::log_activity::LogType::UploadGameAchievementImage { game_id: id, id: uuid }
    };
    _ = log.new(&db).await.map_err(|e|{
        log::error!("Could not save log. {e}");
    });

    Ok(
        Status::Ok
    )
}