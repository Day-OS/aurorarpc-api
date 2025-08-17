use anyhow::Ok;
use reqwest::StatusCode;
use x360connect_global::{activity::Activity, schm_achivements, schm_game};
use std::fmt::format;
use std::future::Future;
use std::pin::Pin;

pub async fn get_schm_game(xbox_url: &String, token: &String) -> anyhow::Result<schm_game::SchmGame>{
    log::debug!("Getting game data from Nova");

    let client = reqwest::Client::new();
    let url = xbox_url.clone() + "/title/live/cache"; 
    let resp = client
        .get(url.clone())
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status() != StatusCode::OK {
        log::error!("Could not gather game data from {url} - Status code: {}", resp.status());
        return Err(anyhow::anyhow!(resp.status()));
    }

    Ok(resp.json::<schm_game::SchmGame>().await?)
}

pub async fn get_schm_achievement(xbox_url: &String, token: &String) -> anyhow::Result<Vec<schm_achivements::Achievement>>{
    log::debug!("Getting achievement data from Nova");

    let client = reqwest::Client::new();
    let resp = client
        .get(xbox_url.clone() + "/achievement")
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status() != StatusCode::OK {
        if resp.status() == StatusCode::NO_CONTENT{
            return Ok(vec![]);
        }
        return Err(anyhow::anyhow!(resp.status()));
    }

    Ok(resp.json::<Vec<schm_achivements::Achievement>>().await?)
}

pub async fn get_icon_achievement(xbox_url: String, uuid: u16, token: &String) -> anyhow::Result<bytes::Bytes>{
    log::debug!("Getting achievemnt icon from Nova");

    let client = reqwest::Client::new();
    let resp = client
        .get(xbox_url.clone() + "/image/achievement")
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

pub async fn feed_game_information(
    game_id: &String, 
    xbox_url: &String, 
    api_url: &String, 
    xbtoken: &String, 
    api_token: &String) -> anyhow::Result<()>{
    let game: schm_game::SchmGame = get_schm_game(xbox_url, xbtoken).await?;
    let achievements = get_schm_achievement(xbox_url, xbtoken).await?;
    log::debug!("Starting to feed game information");
    let client = reqwest::Client::new();
    let game_json = serde_json::to_string(&game)?;
    let url = api_url.clone() + "/game_upload/" + &game_id;
    let resp = client
        .post(url.clone())
        .header("Content-Type", "application/json")
        .bearer_auth(api_token)
        .body(game_json)
        .send()
        .await.map_err(|e|{
            log::error!("Could not send game_upload request at url {url} - {e}");
            e
        })?;
    
    if resp.status() != StatusCode::OK{
        return Err(anyhow::anyhow!(resp.status()));
    }

    for achievement in achievements{
        let achievement_json = serde_json::to_string(&achievement)?;

        let icon = get_icon_achievement(xbox_url.clone(), achievement.imageid.clone(), xbtoken).await?;

        let resp = client
        .post(api_url.clone() + "/achievement_upload/" + &game_id)
        .header("Content-Type", "application/json")
        .bearer_auth(api_token)
        .body(achievement_json)
        .send()
        .await?;
        if resp.status() != StatusCode::OK{
            log::warn!("Got {} while uploading achievement", resp.status());
            continue;
        }
        let image_id = achievement.imageid.to_string();
        let resp = client
        .post(api_url.clone() + "/achievement_upload_i/" + &game_id + "/" + &image_id)
        .header("Content-Type", "image/png")
        .bearer_auth(api_token)
        .body(icon)
        .send()
        .await?;
        if resp.status() != StatusCode::OK{
            log::warn!("Got {} while uploading image for achievement {}", resp.status(), image_id);
        }

    }
    



    Ok(())

}



pub fn get_activity_information(
    game_id: &String,
    xbox_url: &String,
    api_url: Option<String>,
    xbtoken: String,
    apitoken: Option<String>
) -> Pin<Box<dyn Future<Output = anyhow::Result<Activity>> + Send>> {
    log::debug!("Getting information about title {}", game_id);

    let game_id = game_id.clone();
    let xbox_url = xbox_url.clone();
    let api_url = api_url.clone();

    Box::pin(async move {
        // First try to get it from the website 
        if let Some(api_url) = api_url{
            let client = reqwest::Client::new();
            let resp = client
                .get(api_url.clone() + "/game/" + &game_id)
                .send()
                .await?;

            // Verify if the connection was well made and the game exists
            let status = resp.status();
            if status == StatusCode::OK {
            log::debug!("Title {} was found in the API registries", game_id);

                //In the case the game was correctly found, return the activity generated
                let mut result = resp.json::<Activity>().await?;
                result.icon = format!("{api_url}/file/{}",result.icon);
                return Ok(result);
            }

            //In the eventual case the game was not feeded to the api yet, we feed it!
            if status == StatusCode::NOT_FOUND {
            log::debug!("Title {} was NOT found in the API registries.\nFeeding game information to the API", game_id);

                feed_game_information(&game_id.to_owned(), &xbox_url, &api_url, &xbtoken.clone(), &apitoken.clone().expect("API Token should be provided")).await?;
                // and try again!
                return get_activity_information(&game_id.to_owned(), &xbox_url, Some(api_url), xbtoken, apitoken).await;
            }
        }
        
        // This is the case were any other issue happens, so we just assume
        // the server is not operational, we do it locally!
        log::debug!("API could not be reached, showing local data.");

        let game: schm_game::SchmGame = get_schm_game(&xbox_url, &xbtoken).await?;
        let mut icon = None;
        if let Some(images) = game.images {
            if let Some(_icon) = images.icon {
                icon = Some(_icon);
            }
        }
        let icon = icon.unwrap_or("not-found-game".to_owned());
        let activity = Activity {
            title: game.fulltitle.unwrap_or("Undefined".to_owned()),
            icon,
            player: None,
        };
        Ok(activity)
    })
}