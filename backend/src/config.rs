use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        Ok(Config {
            client_id: env::var("TWITCH_CLIENT_ID")?,
            client_secret: env::var("TWITCH_CLIENT_SECRET")?,
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
