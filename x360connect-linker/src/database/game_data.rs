use anyhow::Ok;
use reqwest::StatusCode;
use x360connect_global::{DEFAULT_AVATAR_IMAGE, DEFAULT_BIG_IMAGE};
use x360connect_global::{activity::Activity, schm_achivements, schm_game};
use std::future::Future;
use std::pin::Pin;

use crate::database::profile_data::feed_profile_information;

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
    game_id: &i64, 
    xbox_url: &String, 
    api_url: &String, 
    xbtoken: &String, 
    api_key: &String
) -> anyhow::Result<()>{
    log::debug!("Checking if the game is missing achievements.");
    let client = reqwest::Client::new();
    let url = api_url.clone() + "/achievement_upload/" + &game_id.to_string();
    let resp = client
        .get(url.clone())
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .send()
        .await.map_err(|e|{
            log::error!("Could not verify if the game needs achievements to be uploaded{url} - {e}");
            e
        })?;
    
    let status = resp.status();

    if status == StatusCode::OK{
        return Ok(())
    }
    
    if status != StatusCode::NO_CONTENT{
        return Err(anyhow::anyhow!(status))
    }
    let achievements = get_schm_achievement(xbox_url, xbtoken).await?;

    for achievement in achievements{
        let achievement_json = serde_json::to_string(&achievement)?;

        let icon = get_icon_achievement(xbox_url.clone(), achievement.imageid.clone(), xbtoken).await?;

        let resp = client
        .post(api_url.clone() + "/achievement_upload/" + &game_id.to_string())
        .header("Content-Type", "application/json")
        .bearer_auth(api_key)
        .body(achievement_json)
        .send()
        .await?;
        if resp.status() != StatusCode::OK{
            log::warn!("Got {} while uploading achievement", resp.status());
            continue;
        }
        let image_id = achievement.imageid.to_string();
        let resp = client
        .post(api_url.clone() + "/achievement_upload_i/" + &game_id.to_string() + "/" + &image_id)
        .header("Content-Type", "image/png")
        .bearer_auth(api_key)
        .body(icon)
        .send()
        .await?;
        if resp.status() != StatusCode::OK{
            log::warn!("Got {} while uploading image for achievement {}", resp.status(), image_id);
        }

    }
    



    Ok(())

}

fn generic_activity(title_id: String) -> Activity{
    Activity { title: title_id, icon: DEFAULT_BIG_IMAGE.to_owned(), player: None }
}

pub fn get_activity_information(
    game_id: &i64,
    xbox_url: &String,
    api_url: Option<String>,
    xbtoken: &String,
    apikey: Option<String>
) -> Pin<Box<dyn Future<Output = anyhow::Result<Activity>> + Send>> {
    log::debug!("Getting information about title {}", game_id);

    let game_id = game_id.clone();
    let xbox_url = xbox_url.clone();
    let xbtoken = xbtoken.clone();
    let api_url = api_url.clone();

    Box::pin(async move {
        // First try to get it from the website 
        if let Some(api_url) = api_url{
            let client = reqwest::Client::new();
            let mut builder = client.get(api_url.clone() + "/game/" + &game_id.to_string());
            if let Some(apikey) = apikey.clone() {
                builder = builder.bearer_auth(apikey);
            }
            let resp = builder.send()
                .await?;

            // Verify if the connection was well made and the game exists
            let status = resp.status();
            if status == StatusCode::OK {
                log::debug!("Title {} was found in the API registries", game_id);

                // Attempt to fill achievements in case it does not have
                if let Some(key) = apikey.clone(){
                    feed_game_information(&game_id.to_owned(), &xbox_url, &api_url, &xbtoken.clone(), &key.clone()).await?;
                }


                //In the case the game was correctly found, return the activity generated
                let mut result = resp.json::<Activity>().await?;
                if result.icon != DEFAULT_BIG_IMAGE{
                    result.icon = format!("{api_url}/file/{}",result.icon);
                }
                if let Some(mut player) = result.player {
                    if player.picture != DEFAULT_AVATAR_IMAGE{
                        player.picture = format!("{api_url}/file/{}", player.picture);
                    }
                    player.url = format!("{api_url}{}", player.url);
                    result.player = Some(player);
                }

                // If user have an access key, then try to update their profiles with all logged ones
                if let Some(apikey) = apikey.clone(){
                    feed_profile_information(&game_id.to_string(), &xbox_url, &api_url, &xbtoken, &apikey).await?;
                }
                

                return Ok(result);
            }

            //In the eventual case the game was not feeded to the api yet, we feed it!
            if status == StatusCode::NOT_FOUND {
                log::debug!("Title {} was NOT found in the API registries.", game_id);
                return Ok(generic_activity(game_id.to_string()))
            }
        }
        
        // This is the case were any other issue happens, so we just assume
        // the server is not operational, we do it locally!
        log::debug!("API could not be reached.");

        return Ok(generic_activity(game_id.to_string()))
    })
}