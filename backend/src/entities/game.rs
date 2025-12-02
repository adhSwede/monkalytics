#[derive(Debug)]
pub struct Game {
    pub id: Option<i32>,
    pub game_id: String,
    pub name: String,
    pub box_art_url: String,
    pub igdb_id: Option<String>,
    pub polled_at: Option<String>,
}
