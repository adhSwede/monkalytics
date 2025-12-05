use crate::entities::game::Game;
use sqlx::PgPool;

pub async fn insert_game(pool: &PgPool, game: &Game) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO games (
            game_id,
            name,
            box_art_url,
            igdb_id
        ) VALUES ($1, $2, $3, $4)
        ON CONFLICT (game_id) DO NOTHING
        "#,
    )
    .bind(&game.game_id)
    .bind(&game.name)
    .bind(&game.box_art_url)
    .bind(&game.igdb_id)
    .execute(pool)
    .await?;

    Ok(())
}
