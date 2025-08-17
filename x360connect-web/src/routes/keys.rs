use rocket_db_pools::Connection;
use rocket::http::Status;

use crate::{user::model::{User}, MongoDB};
use crate::access_key::AccessKeyGuard;

#[get("/keys/verify")]
pub async fn verify_key(
    access_key: AccessKeyGuard,
    db: Connection<MongoDB>
) -> Status {
    match User::find_user_by_key(&db, access_key.0).await {
        Ok(user) =>{
            match user {
                Some(_) => Status::Ok,
                None => Status::Forbidden,
            }
        },
        Err(e) => {
            log::error!("{e}");
            Status::InternalServerError
        },
    }

    // let user = User::get_from_cookie(&db, cookies).await.unwrap();
}