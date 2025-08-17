use std::collections::HashMap;

use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct TokenResponse{
    token: String,
}

pub async fn get_token(url: String, username: &str, password: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("username", &username);
    params.insert("password", &password);
    let resp = client
        .post(url + "/authenticate")
        .form(&params)
        .send()
        .await?;

    let status = resp.status();

    if status != StatusCode::OK{
        if status == StatusCode::UNAUTHORIZED{
            log::info!("Nova's username or password seems to be wrong.\nVerify the correct username and password at Aurora Home")
        }
        return Err(anyhow::anyhow!(status));
    }

    let resp= resp.text()
        .await?;
    let resp: TokenResponse = serde_json::from_str(&resp).map_err(|e|{
        log::error!("An error ocurred while trying to get an authentication token\n{e}\n{}", resp);
        e
    })?;
    Ok(resp.token)
}