use anyhow::Ok;
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};


pub async fn verify_key(api_url: String, key: String) -> anyhow::Result<bool> {    
    let client = reqwest::Client::new();
    
    let mut map = HeaderMap::new();
    map.insert("x-access-key", HeaderValue::from_str(&key)?);
    
    let status_code: reqwest::StatusCode = client
        .get(api_url.clone() + "/keys/verify")
        .headers(map)
        .send()
        .await?
        .status();

    if status_code == StatusCode::OK{
        return Ok(true);
    }
    else if status_code == StatusCode::FORBIDDEN{
        return Ok(false)
    }
    else{
        return Err(anyhow::anyhow!(status_code));
    }
}