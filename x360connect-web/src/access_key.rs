use rocket::{http::Status, request::{FromRequest, Outcome}, Request};

pub async fn get_key<'r>(req: &'r Request<'_>) -> Option<String> {
    req.headers().get_one("x-access-key").map(|string| string.to_string())
}

pub struct AccessKeyGuard(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match get_key(req).await {
            Some(key) => Outcome::Success(AccessKeyGuard(key)),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}


pub struct OptionalAccessKeyGuard(pub Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OptionalAccessKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(OptionalAccessKeyGuard(get_key(req).await))
    }
}

