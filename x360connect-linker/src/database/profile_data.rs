use anyhow::Ok;
use reqwest::StatusCode;
use x360connect_global::schm_profile::{PlayersAchievements, ProfileBasic, SchmProfile, SchmProfileUploadResponse};


pub async fn get_basic_profiles(xbox_url: &String, token: &String) -> anyhow::Result<Vec<ProfileBasic>>{
    log::debug!("Getting logged in profiles from Nova");

    let client = reqwest::Client::new();
    let url = xbox_url.clone() + "/profile"; 
    let resp = client
        .get(url.clone())
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status() != StatusCode::OK {
        log::error!("Could not gather profiles from {url} - Status code: {}", resp.status());
        return Err(anyhow::anyhow!(resp.status()));
    }

    Ok(resp.json::<Vec<ProfileBasic>>().await?)
}

pub async fn get_profile_achievements(xbox_url: &String, token: &String) -> anyhow::Result<Vec<PlayersAchievements>>{
    log::debug!("Getting achievements from running game from Nova");

    let client = reqwest::Client::new();
    let url = xbox_url.clone() + "/achievement/player"; 
    let resp = client
        .get(url.clone())
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status() == StatusCode::NO_CONTENT{
        log::debug!("No achievements found for the curent game");
        return Ok(vec![])
    }

    if resp.status() != StatusCode::OK {
        log::error!("Could not gather achievements from {url} - Status code: {}", resp.status());
        return Err(anyhow::anyhow!(resp.status()));
    }

    Ok(resp.json::<Vec<PlayersAchievements>>().await?)
}

pub async fn get_icon_profile(xbox_url: &String, uuid: u8, token: &String) -> anyhow::Result<bytes::Bytes>{
    log::debug!("Getting profile icon from Nova");

    let client = reqwest::Client::new();
    let resp = client
        .get(xbox_url.clone() + "/image/profile")
        .query(&vec![("uuid", uuid)])
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(resp.status()));
    }
    let image = resp.bytes().await?;

    Ok(image)
}



pub async fn feed_profile_information(
    game_id: &String, 
    xbox_url: &String, 
    api_url: &String, 
    xbtoken: &String, 
    api_token: &String
) -> anyhow::Result<()>{

    let mut profiles: Vec<ProfileBasic> = get_basic_profiles(xbox_url, xbtoken).await?;

    // Filter by signed in only
    // (Nova gives us the empty spaces aswell, so we need to filter them out)
    profiles = profiles.iter().filter(|profile| {
        profile.signedin == 1
    }).cloned().collect();
    
    let achievements = get_profile_achievements(xbox_url, xbtoken).await?;

    let profile_body: SchmProfile = SchmProfile::new(game_id.to_string(), achievements, profiles);

    log::debug!("Starting to upload profiles");
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&profile_body)?;
    let url = api_url.clone() + "/profile_upload/";
    let resp = client
        .post(url.clone())
        .header("Content-Type", "application/json")
        .bearer_auth(api_token)
        .body(body)
        .send()
        .await.map_err(|e|{
            log::error!("Could not send profile_upload request at url {url} - {e}");
            e
        })?;
    
    if resp.status() != StatusCode::OK{
        return Err(anyhow::anyhow!(resp.status()));
    }

    let response = resp.json::<SchmProfileUploadResponse>().await?;

    // We now get the profiles indexes that are required to send a new version of its gamerpic
    let mut need_update_profile = vec![];

    for profile in &profile_body.profiles {
        for xuid_that_needs_update in &response.needs_picture_update{
            if profile.base.xuid == *xuid_that_needs_update{
                need_update_profile.push(profile.clone());
            }
        }
    }

    // and now upload the new pictures
    for profile in need_update_profile{
        let image = get_icon_profile(xbox_url, profile.base.index, xbtoken).await?;

        let image = image::load_from_memory_with_format(&image, image::ImageFormat::Bmp)?;
        let mut new_image = std::io::Cursor::new(Vec::new());

        image.write_to(&mut new_image, image::ImageFormat::Png)?;
        let new_image = new_image.into_inner();
        let resp = client
        .post(api_url.clone() + "/profile_upload_i/" + &profile.base.xuid)
        .header("Content-Type", "image/png")
        .bearer_auth(api_token)
        .body(new_image)
        .send()
        .await?;
        if resp.status() != StatusCode::OK{
            log::warn!("Got {} while uploading profile picture", resp.status());
        }
    }
    Ok(())
}