#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GameInfo {
    pub path: String,
    pub titleid: String,
    pub mediaid: String,
    pub disc: DiscInfo,
    pub tuver: u32,
    pub version: VersionInfo,
    pub resolution: ResolutionInfo,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DiscInfo {
    pub current: u32,
    pub count: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VersionInfo {
    pub base: String,
    pub current: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ResolutionInfo {
    pub width: u32,
    pub height: u32,
}

pub async fn get_title(url: String, token: String) -> anyhow::Result<GameInfo> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url + "/title")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    let resp: GameInfo = serde_json::from_str(&resp)?;
    Ok(resp)
}