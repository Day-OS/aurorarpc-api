use ordermap::OrderSet;
use rocket::{data::ToByteUnit, futures::AsyncWriteExt, http::Status, serde::json::Json, tokio::io::AsyncReadExt, Data};
use rocket_db_pools::Connection;
use x360connect_global::{schm_achivements, schm_game::SchmGame};

use crate::{access_key::AccessKeyGuard, game::model::{Achievement, Game}, log_activity::Log, user::model::User, MongoDB, DATABASE_NAME};

#[post("/game_upload/<id>", data = "<input>")]
pub async fn game_upload<'r>(
    id: &str,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    input: Json<SchmGame>
) -> Result<Status, Status> {
   
    let user = User::find_user_by_key(&db, access_key.0).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::Forbidden)?;

    let game_already_exists = Game::find_by_id(&db, id.to_owned())
        .await.map_err(|e|{
            error!("{e}");
            Status::InternalServerError
        })?;
    
    if game_already_exists.is_some(){
        return Err(Status::Conflict);
    }

    let mut game = Game{ 
        id: None, 
        game_id: id.to_string(), 
        schema: input.into_inner(), 
        achievements: OrderSet::new()
    };

    game.upload_own_images(&db).await.map_err(|e|{
        error!("Error while trying to change the source of the images from game {} - {e}", game.get_name());
        Status::InternalServerError
    })?;

    _ = game.new(&db).await.map_err(|e|{
        log::error!("Could not save game. {e}");
    });

    let log = Log{ 
        id: None, 
        discord_id: user.discord_id, 
        log_type: crate::log_activity::LogType::UploadGameInfo { game_id: id.to_string() } 
    };
    _ = log.new(&db).await.map_err(|e|{
        log::error!("Could not save log. {e}");
    });
    

    Ok(
        Status::Ok
    )
}

#[post("/achievement_upload/<id>", data = "<input>")]
pub async fn achievement_upload<'r>(
    id: String,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    input: Json<schm_achivements::Achievement>
) -> Result<Status, Status> {
   
    let user = User::find_user_by_key(&db, access_key.0).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::Forbidden)?;

    let game_already_exists = Game::find_by_id(&db, id.to_owned())
        .await.map_err(|e|{
            error!("{e}");
            Status::InternalServerError
        })?;
    
    if game_already_exists.is_none(){
        return Err(Status::NotFound);
    }

    let mut game = game_already_exists.unwrap();
    let image_id = input.0.imageid.to_string();
    let url = game.achivement_image_name(image_id.clone());

    let achievement = Achievement{ 
        id: input.0.id.clone(), 
        schema: input.into_inner(), 
        icon_url: url
    };

    game.achievements.insert(achievement);

    _ = game.save(&db).await.map_err(|e|{
        log::error!("Could not save game. {e}");
    });

    let log = Log{ 
        id: None, 
        discord_id: user.discord_id, 
        log_type: crate::log_activity::LogType::UploadGameAchievementInfo { game_id: id, id: image_id }
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
    id: String,
    uuid: String,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    image: Data<'_>
) -> Result<Status, Status> {
   
    let user = User::find_user_by_key(&db, access_key.0).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::Forbidden)?;

    let mut buf = Vec::new();
    image.open(10.mebibytes()).read_to_end(&mut buf).await.unwrap();

    let game = Game::find_by_id(&db, id.clone()).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::NotFound)?;

    let file_name = game.achivement_image_name(uuid.clone());

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