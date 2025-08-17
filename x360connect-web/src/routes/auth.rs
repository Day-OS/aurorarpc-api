use rocket::http::CookieJar;
use oauth2::{
    AuthorizationCode, TokenResponse
};
use rocket_db_pools::{mongodb::bson::doc, Connection};

use crate::{user::{authenticate::{get_discord_info, oauth_client}, model::{User}}, MongoDB};


#[get("/auth?<code>&<_state>")]
pub async fn auth(
    code: String, 
    _state: Option<String>, 
    cookies: &CookieJar<'_>,
    db: Connection<MongoDB>
) -> Result<String, String> {
    let client = oauth_client();

    let http_client = reqwest::Client::new();
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&http_client)
        .await
        .map_err(|e| format!("Erro ao trocar código: {:?}", e))?;

    let access_token = token_result.access_token().secret();

    let user = get_discord_info(access_token).await.unwrap();

    let mut db_user = User::find_by_discord_id(&db, user.id.clone()).await.unwrap();
    
    if db_user.is_none(){
        db_user = User::new(&db, user.id.clone(), user.username.clone()).await.ok();
    }

    let db_user = db_user.unwrap();

    cookies.add_private(rocket::http::Cookie::new("discord_user_id", user.id.clone()));

    Ok(format!("Hello {}", db_user.nickname))
}