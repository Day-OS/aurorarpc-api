use std::collections::HashMap;

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
    //let resp = reqwest::get("http://192.168.1.27:9999/title").await?.text().await?;
    let resp = client
        .post(url + "/authenticate")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    let resp: TokenResponse = serde_json::from_str(&resp)?;
    Ok(resp.token)
}