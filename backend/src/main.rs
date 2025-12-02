mod api;
mod config;
mod db;
mod dtos;

use api::twitch::{fetch_top_games, fetch_top_streams, get_access_token};
use config::Config;
use db::connection::create_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let pool = create_pool(&config.database_url).await?;
    println!("✓ Database connected");

    db::schema::init_tables(&pool).await?;

    let access_token = get_access_token(&config.client_id, &config.client_secret).await?;

    let top_streams = fetch_top_streams(&config.client_id, &access_token).await?;
    println!("Fetched {} streams", top_streams.data.len());

    let top_games = fetch_top_games(&config.client_id, &access_token).await?;
    println!("Fetched {} games", top_games.data.len());

    println!("\nTop streams:");
    println!("{:#?}", top_streams);

    println!("\nTop games:");
    println!("{:#?}", top_games);

    Ok(())
}
