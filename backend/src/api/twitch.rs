use reqwest;
use serde_json;

pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let token_res = reqwest::Client::new()
        .post("https://id.twitch.tv/oauth2/token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let access_token = token_res["access_token"].as_str().unwrap().to_string();

    Ok(access_token)
}

pub async fn fetch_top_streams(
    client_id: &str,
    access_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let stream_res = reqwest::Client::new()
        .get("https://api.twitch.tv/helix/streams")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(stream_res)
}
