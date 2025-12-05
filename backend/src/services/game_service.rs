use crate::api::twitch::fetch_top_games;
use crate::entities::game::Game;
use crate::repositories::game_repository::insert_game;
use sqlx::PgPool;

pub async fn fetch_and_store_games(
    pool: &PgPool,
    client_id: &str,
    access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let games_response = fetch_top_games(client_id, access_token).await?;
    let games_count = games_response.data.len();

    for dto in games_response.data {
        let game = Game {
            id: None,
            game_id: dto.id,
            name: dto.name,
            box_art_url: dto.box_art_url,
            igdb_id: dto.igdb_id,
            polled_at: None,
        };

        insert_game(pool, &game).await?
    }

    println!("✓ Stored {} games", games_count);
    Ok(())
}
