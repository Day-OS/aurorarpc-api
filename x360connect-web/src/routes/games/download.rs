use rocket::{futures::AsyncReadExt, http::{ContentType, Status}, response::status::Custom};
use rocket_db_pools::{mongodb::bson, Connection};

use crate::{game::model::Game, MongoDB, DATABASE_NAME};


#[get("/achievement/<id>/<uuid>")]
pub async fn get_achievement<'r>(
    id: String,
    uuid: String,
    db: Connection<MongoDB>,
) -> Result<(ContentType, Vec<u8>), Status> {
   

    let game = Game::find_by_id(&db, id).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?.ok_or(Status::NotFound)?;

    let file_name = game.achivement_image_name(uuid);

    let mut buf: Vec<u8> = vec![];

    let bucket = db.database(DATABASE_NAME).gridfs_bucket(None);
    let mut upload_stream = bucket.open_download_stream_by_name(file_name, None)
    .await.map_err(|e|{
        error!("{e}");
        Status::InternalServerError
    })?;
    upload_stream.read_to_end(&mut buf).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;

    Ok( (ContentType::PNG, buf))
}