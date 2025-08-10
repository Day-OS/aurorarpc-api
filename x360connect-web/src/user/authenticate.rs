use std::env;

use anyhow::Ok;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl
};
use serde::Deserialize;

const DISCORD_AUTH_URL: &str = "https://discord.com/api/oauth2/authorize";
const DISCORD_TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const DISCORD_API_URL: &str = "https://discord.com/api/users/@me";


#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    // pub avatar: Option<String>,
}


pub fn oauth_client() -> oauth2::Client<oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>, oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::StandardTokenIntrospectionResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::StandardRevocableToken, oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>, oauth2::EndpointSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointNotSet, oauth2::EndpointSet>  {
    const ERROR_MSG: &'static str = "variable must be provided";

    let client_id = env::var("OAUTH_ID").expect(&format!("OAUTH_ID {ERROR_MSG}"));
    let secret_id: String = env::var("OAUTH_SECRET").expect(&format!("OAUTH_SECRET {ERROR_MSG}"));

    BasicClient::new(
        ClientId::new(client_id),
        // Some(ClientSecret::new("SEU_CLIENT_SECRET".to_string())),
        // AuthUrl::new(DISCORD_AUTH_URL.to_string()).expect("Invalid auth URL"),
        // Some(TokenUrl::new(DISCORD_TOKEN_URL.to_string()).expect("Invalid token URL")),
    ).set_client_secret(ClientSecret::new(secret_id))
    .set_auth_uri(AuthUrl::new(DISCORD_AUTH_URL.to_owned()).unwrap())
    .set_token_uri(TokenUrl::new(DISCORD_TOKEN_URL.to_owned()).unwrap())
    // Seu redirect URL precisa estar cadastrado no Discord Dev Portal
    .set_redirect_uri(RedirectUrl::new("http://localhost:8000/auth".to_string()).expect("Invalid redirect URL"))
}

pub async fn get_discord_info(access_token: &String) -> anyhow::Result<DiscordUser> {
    // Pega infos do usuário no Discord
    let user_resp = reqwest::Client::new()
        .get(DISCORD_API_URL)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !user_resp.status().is_success() {
        return Err(anyhow::anyhow!("Erro HTTP ao buscar user info: {}", user_resp.status()));
    }

    let user: DiscordUser = serde_json::from_str(&user_resp
        .text().await?)?;
    Ok(user)
}