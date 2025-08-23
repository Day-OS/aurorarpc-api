use rocket::http::Status;

use crate::access_key::AccessKeyGuard;
use crate::modules::user::model::User;

#[get("/keys/verify")]
pub async fn verify_key(
    access_key: AccessKeyGuard,
) -> Status {
    let user: User = access_key.0;
    log::info!("User {} verified!", user.username);
    Status::Ok
}