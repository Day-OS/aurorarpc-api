use rocket::{futures::AsyncReadExt, http::{ContentType, Status}};
use rocket_db_pools::Connection;

use crate::{MongoDB, DATABASE_NAME};


#[get("/file/<file_name>")]
pub async fn get_file<'r>(
    file_name: String,
    db: Connection<MongoDB>,
) -> Result<(ContentType, Vec<u8>), Status> {
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