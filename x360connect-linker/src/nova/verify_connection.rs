const CHECKER: &str = "<title>NOVA webUI</title>";

pub async fn verify_connection(url: String) -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(resp.contains(CHECKER))
}