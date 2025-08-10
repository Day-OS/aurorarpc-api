use serde::{Deserialize, Serialize};
use x360connect_global::activity::Activity;



pub async fn get_game_information(game_id: String, api_url: String) -> anyhow::Result<Activity> {
    let game_id = &game_id[2..];
    
    let client = reqwest::Client::new();
    //let resp = reqwest::get("http://192.168.1.27:9999/title").await?.text().await?;
    let resp = client
        .get(api_url.clone() + "/game/" + &game_id)
        .send()
        .await?
        .text()
        .await?;

    let mut resp: Activity = serde_json::from_str(&resp)?;
    resp.icon = format!("{api_url}/{}", resp.icon);
    Ok(resp)
}