use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TwitchGameDto {
    pub id: String,
    pub name: String,
    pub box_art_url: String,
    pub igdb_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TwitchGamesResponse {
    pub data: Vec<TwitchGameDto>,
}
