#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ProfileResponse {
    pub index: u32,
    pub gamertag: String,
    pub gamerscore: u32,
    pub signedin: u8,
    pub xuid: String,
}

pub struct Profile {
    pub base: ProfileResponse,
    pub avatar: String,
}



pub async fn get_avatar(url: String, token: String) -> anyhow::Result<Profile> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url.clone() + "/profile")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    let profiles: Vec<ProfileResponse> = serde_json::from_str(&resp)?;

    Ok(Profile{
        base: profiles.first().unwrap().clone(),
        avatar: format!("{url}")
    })
}