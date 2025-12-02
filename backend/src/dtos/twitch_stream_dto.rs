use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwitchStreamDto {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: String,
    pub game_name: String,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub title: String,
    pub viewer_count: i32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String,
    pub tags: Vec<String>,
    pub is_mature: bool,
}

#[derive(Debug, Deserialize)]
pub struct TwitchStreamsResponse {
    pub data: Vec<TwitchStreamDto>,
}
