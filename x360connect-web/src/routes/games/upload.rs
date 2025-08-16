use rocket::{data::ToByteUnit, futures::AsyncWriteExt, http::Status, serde::json::Json, tokio::io::AsyncReadExt, Data};
use rocket_db_pools::Connection;
use x360connect_global::schm_game::SchmGame;

use crate::{access_key::AccessKeyGuard, game::model::Game, user::model::User, MongoDB, DATABASE_NAME};

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

    

    Ok(
        Status::Ok
    )
}

#[post("/achievement_upload/<id>", data = "<input>")]
pub async fn achievement_upload<'r>(
    id: String,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    input: Json<SchmGame>
) -> Result<Status, Status> {
   
    let user = User::find_user_by_key(&db, access_key.0).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::Forbidden)?;
    let mut picture = "assets/default_image.png".to_owned();

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

    let game = Game::find_by_id(&db, id).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::NotFound)?;

    let file_name = game.achivement_image_name(uuid);

    let bucket = db.database(DATABASE_NAME).gridfs_bucket(None);
    let mut upload_stream = bucket.open_upload_stream(file_name, None);
    upload_stream.write_all(&buf).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;

    Ok(
        Status::Ok
    )
}