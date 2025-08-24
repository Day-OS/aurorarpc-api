use std::collections::HashMap;

use rocket::{data::ToByteUnit, futures::AsyncWriteExt, http::Status, serde::json::Json, tokio::io::AsyncReadExt, Data};
use rocket_db_pools::Connection;
use x360connect_global::{schm_profile::{SchmProfile, SchmProfileUploadResponse}, DEFAULT_AVATAR_IMAGE};

use crate::{access_key::AccessKeyGuard, modules::user::model::Profile, MongoDB, DATABASE_NAME};

#[post("/profile_upload", data = "<input>")]
pub async fn profile_upload<'r>(
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    input: Json<SchmProfile>
) -> Result<Json<SchmProfileUploadResponse>, Status> {
   
    let mut user = access_key.0;

    let mut profiles_that_needs_picture_update = vec![];

    let profile_input = input.0;
    let current_game_id = i64::from_str_radix(&profile_input.current_game, 16).unwrap();

    // Go through the list and check who needs an update
    for in_profile in profile_input.profiles{
        let xuid = in_profile.base.xuid.clone();

        match user.profiles.get_mut(&xuid) {
            Some(profile) => {
                profile.gamerscore = in_profile.base.gamerscore;
                // In case the profile is already registered, just update the game record data
                profile.game_record.insert(current_game_id.clone().to_string(), in_profile.achievements);

                // Does this logged avatar need to have its picture updated?
                if profile.needs_picture_update{
                    // Then return to the linker an answer asking it to send the new picture!
                    profiles_that_needs_picture_update.push(xuid.clone());
                }
            },
            None => {
                let mut game_record = HashMap::new();
                game_record.insert(current_game_id.clone().to_string(), in_profile.achievements);
                // In case the profile is unknown, generate it and mark it as in need to be updated
                let new_profile = Profile { 
                    avatar_url: DEFAULT_AVATAR_IMAGE.to_string(), 
                    gamertag: in_profile.base.gamertag,
                    gamerscore: in_profile.base.gamerscore,
                    needs_picture_update: true,
                    game_record: game_record
                };
                profiles_that_needs_picture_update.push(xuid.clone());

                user.profiles.insert(xuid.clone(), new_profile);
            },
        }

        if user.selected_profile.is_none(){
            user.selected_profile = Some(xuid.clone());
        }
    }


    user.save(&db).await.map_err(|e|{
        log::error!("Could not save user after profile upload {e}");
        Status::InternalServerError
    })?;

    Ok(
        rocket::serde::json::Json(
            SchmProfileUploadResponse{ needs_picture_update: profiles_that_needs_picture_update }
        )
    )
}


#[post("/profile_upload_i/<xuid>", data = "<image>")]
pub async fn profile_upload_i<'r>(
    xuid: String,
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>,
    image: Data<'_>
) -> Result<Status, Status> {
   
    let mut user = access_key.0;

    let profile_opt = user.profiles.get(&xuid);

    if profile_opt.is_none(){
        return Err(Status::NoContent)
    }

    let mut profile = profile_opt.unwrap().clone();
    

    let mut buf = Vec::new();
    image.open(10.mebibytes()).read_to_end(&mut buf).await.unwrap();

    let file_name: String = user.profile_image_name(xuid.clone());

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

    
    profile.needs_picture_update = false;
    profile.avatar_url = user.profile_image_name(xuid.clone());
    user.profiles.insert(xuid, profile);

    user.save(&db).await.map_err(|e|{
        log::error!("{e}");
        Status::InternalServerError
    })?;

    Ok(
        Status::Ok
    )
}