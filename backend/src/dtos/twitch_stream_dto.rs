use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwitchStreamDto {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: Option<String>,
    pub game_name: Option<String>,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub title: Option<String>,
    pub viewer_count: i32,
    pub started_at: Option<String>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub tag_ids: Option<Vec<String>>, // Change to Option
    pub tags: Option<Vec<String>>,    // Change to Option
    pub is_mature: bool,
}

#[derive(Debug, Deserialize)]
pub struct TwitchStreamsPagination {
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchStreamsResponse {
    pub data: Vec<TwitchStreamDto>,
    pub pagination: Option<TwitchStreamsPagination>,
}
