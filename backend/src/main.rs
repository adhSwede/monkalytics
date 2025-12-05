mod api;
mod config;
mod db;
mod dtos;
mod entities;
mod repositories;
mod services;

use api::twitch::get_access_token;
use config::Config;
use db::connection::create_pool;
use services::game_service::fetch_and_store_games;
use services::stream_service::fetch_and_store_streams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let pool = create_pool(&config.database_url).await?;
    println!("✓ Database connected");

    db::schema::init_tables(&pool).await?;

    let access_token = get_access_token(&config.client_id, &config.client_secret).await?;

    fetch_and_store_games(&pool, &config.client_id, &access_token).await?;
    fetch_and_store_streams(&pool, &config.client_id, &access_token).await?;

    Ok(())
}
