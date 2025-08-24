use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use rocket_db_pools::Database;

use crate::{modules::user::model::User, MongoDB};
pub async fn get_key<'r>(req: &'r Request<'_>) -> Result<Option<User>, Status> {
    let db = MongoDB::fetch(req.rocket());
    if db.is_none(){
        return Err(Status::InternalServerError);
    }
    let db = db.unwrap();

    let token = req.headers()
    .get_one("Authorization")
    .and_then(|header| {
        if let Some(token) = header.strip_prefix("Bearer ") {
            Some(token.to_string())
        } else {
            None
        }
    });

    match token {
        Some(token) => {
            let user = User::find_user_by_key(&db, token.to_string())
                .await
                .map_err(|e|{
                    log::error!("Failed to acquire user: {e}");
                    Status::InternalServerError
                })?;
            Ok(user)
        },
        None => {
            Ok(None)
        },
    }
}

pub struct AccessKeyGuard(pub User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match get_key(req).await{
            Ok(user) => user,
            Err(e) => return Outcome::Error((e, ())),
        };
        match user {
            Some(user) => Outcome::Success(AccessKeyGuard(user)),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}


pub struct OptionalAccessKeyGuard(pub Option<User>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OptionalAccessKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match get_key(req).await{
            Ok(user) => user,
            Err(e) => return Outcome::Error((e, ())),
        };    
        Outcome::Success(OptionalAccessKeyGuard(user))
    }
}

