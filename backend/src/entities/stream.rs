#[derive(Debug)]
pub struct Stream {
    pub id: Option<i32>,
    pub stream_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub game_id: Option<String>,
    pub game_name: Option<String>,
    pub stream_type: String,
    pub title: Option<String>,
    pub viewer_count: i32,
    pub started_at: Option<String>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub tags: Option<String>,
    pub is_mature: bool,
}
