use rocket::response::Redirect;

use oauth2::{
    CsrfToken, Scope
};
use rocket_okapi::openapi;

use crate::modules::user::authenticate::oauth_client;

#[openapi(tag = "Login")]
#[get("/login-req")]
pub async fn login_req() -> Redirect {
    let client = oauth_client();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(auth_url.to_string())
}