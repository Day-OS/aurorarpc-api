use rocket::{http::Status, request::{FromRequest, Outcome}, Request};

pub struct AccessKeyGuard(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-access-key") {
            Some(key) => Outcome::Success(AccessKeyGuard(key.to_string())),
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
